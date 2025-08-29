use std::sync::Arc;

use crate::pages::components::datatable::core::column::ColumnState;
use crate::pages::components::datatable::core::state::TableState;
use leptos::prelude::{Update, With};

thread_local! {
    static ACTIVE_RESIZE_COL: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{window_event_listener, WindowListenerHandle};

pub struct ResizeService<T: Send + Sync + 'static> {
    state: Arc<TableState<T>>, // table state handle
    resizing: bool,            // whether a resize is in progress
    col_id: Option<String>,    // id of the column being resized
    start_x: i32,              // pointer x at begin
    current_x: i32,            // latest pointer x
    start_width: i32,          // effective width at begin (taking overrides into account)
    #[cfg(target_arch = "wasm32")]
    move_guard: std::rc::Rc<std::cell::Cell<Option<WindowListenerHandle>>>,
    #[cfg(target_arch = "wasm32")]
    up_guard: std::rc::Rc<std::cell::Cell<Option<WindowListenerHandle>>>,
}

impl<T: Send + Sync + 'static> ResizeService<T> {
    #[cfg(target_arch = "wasm32")]
    fn attach_listeners(&mut self) {
        // ensure previous listeners are dropped first
        self.detach_listeners();
        // Snapshot what we need so we don't capture &mut self inside 'static closures
        let id = self.col_id.clone().unwrap_or_default();
        let state = self.state.clone();
        let start_x = self.start_x;
        let start_width = self.start_width; // snapshot base width once at begin
        let move_cell = self.move_guard.clone();
        let up_cell = self.up_guard.clone();
        // move listener
        let mv_handle = window_event_listener(leptos::ev::mousemove, {
            let id = id.clone();
            move |e: leptos::ev::MouseEvent| {
                // Only process when mouse button is held and this column is active.
                #[allow(unused_mut)]
                let mut pressed = true;
                #[cfg(target_arch = "wasm32")]
                {
                    pressed = e.buttons() & 1 == 1;
                }
                if !pressed {
                    return;
                }
                let is_active =
                    ACTIVE_RESIZE_COL.with(|c| c.borrow().as_deref() == Some(id.as_str()));
                if !is_active {
                    return;
                }
                // Compute width based on start snapshot only (no reread of base from state)
                let delta = e.client_x() - start_x;
                // Obtain constraints for clamping
                let (min_w, max_w_opt) = state.columns.with(|cols| {
                    if let Some(c) = cols.iter().find(|c| c.id == id.as_str()) {
                        (c.min_width.max(0), c.max_width)
                    } else {
                        (0, None)
                    }
                });
                let mut w = start_width.saturating_add(delta);
                if let Some(maxw) = max_w_opt {
                    w = w.clamp(min_w, maxw);
                } else {
                    w = w.max(min_w);
                }
                // Apply width to state
                state.column_state.update(|m| {
                    let entry = m.entry(id.clone()).or_insert_with(|| ColumnState {
                        id: id.clone(),
                        ..Default::default()
                    });
                    entry.width = Some(w);
                });
            }
        });
        self.move_guard.set(Some(mv_handle));
        let up_handle =
            window_event_listener(leptos::ev::mouseup, move |_e: leptos::ev::MouseEvent| {
                // On mouseup, drop listeners and clear active column; do not touch self
                if let Some(h) = move_cell.take() {
                    drop(h);
                }
                if let Some(h) = up_cell.take() {
                    drop(h);
                }
                ACTIVE_RESIZE_COL.with(|c| c.replace(None));
            });
        self.up_guard.set(Some(up_handle));
    }

    #[cfg(target_arch = "wasm32")]
    fn detach_listeners(&mut self) {
        if let Some(h) = self.move_guard.take() {
            drop(h);
        }
        if let Some(h) = self.up_guard.take() {
            drop(h);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn attach_listeners(&mut self) {}
    #[cfg(not(target_arch = "wasm32"))]
    fn detach_listeners(&mut self) {}
    pub fn new(state: Arc<TableState<T>>) -> Self {
        Self {
            state,
            resizing: false,
            col_id: None,
            start_x: 0,
            current_x: 0,
            start_width: 0,
            #[cfg(target_arch = "wasm32")]
            move_guard: std::rc::Rc::new(std::cell::Cell::new(None)),
            #[cfg(target_arch = "wasm32")]
            up_guard: std::rc::Rc::new(std::cell::Cell::new(None)),
        }
    }

    /// Begin resizing a column: record the starting pointer position and the starting width.
    pub fn begin_resize(&mut self, col_id: &str, start_x: i32) {
        self.resizing = true;
        self.col_id = Some(col_id.to_string());
        // mark this column as the only active resize target globally
        let active_id = col_id.to_string();
        ACTIVE_RESIZE_COL.with(|c| c.replace(Some(active_id)));
        self.start_x = start_x;
        self.current_x = start_x;
        // Determine effective start width (column_state override or column def width)
        let id = col_id.to_string();
        let effective = {
            let def_width = self.state.columns.with(|cols| {
                cols.iter()
                    .find(|c| c.id == id.as_str())
                    .map(|c| c.width)
                    .unwrap_or(0)
            });
            self.state
                .column_state
                .with(|m| m.get(&id).and_then(|cs| cs.width))
                .unwrap_or(def_width)
        };
        self.start_width = effective.max(0);
        // attach global listeners (wasm only)
        self.attach_listeners();
    }

    /// Update the current pointer X during resize (safe when not resizing). Also applies live width while dragging.
    pub fn update_resize(&mut self, current_x: i32) {
        if self.resizing {
            self.current_x = current_x;
            if let Some(id) = self.col_id.clone() {
                let delta = self.current_x - self.start_x;
                let new_w = self.compute_new_width(&id, self.start_width, delta);
                self.apply_width(&id, new_w);
            }
        }
    }

    /// Finish the resize operation, applying the new width into column_state (clamped to min/max).
    pub fn end_resize(&mut self) {
        if !self.resizing {
            return;
        }
        let delta = self.current_x - self.start_x;
        let id_opt = self.col_id.clone();
        if let Some(id) = id_opt {
            let new_w = self.compute_new_width(&id, self.start_width, delta);
            self.apply_width(&id, new_w);
        }
        // reset internal state
        self.resizing = false;
        self.col_id = None;
        self.start_x = 0;
        self.current_x = 0;
        self.start_width = 0;
        // drop listeners
        self.detach_listeners();
        // clear active
        ACTIVE_RESIZE_COL.with(|c| c.replace(None));
    }

    /// Compute a clamped width based on column definition constraints.
    fn compute_new_width(&self, id: &str, base: i32, delta: i32) -> i32 {
        let (min_w, max_w_opt) = self.state.columns.with(|cols| {
            if let Some(c) = cols.iter().find(|c| c.id == id) {
                (c.min_width.max(0), c.max_width)
            } else {
                (0, None)
            }
        });
        let mut w = base.saturating_add(delta);
        if let Some(maxw) = max_w_opt {
            w = w.clamp(min_w, maxw);
        } else {
            w = w.max(min_w);
        }
        w
    }

    /// Apply a width into column_state for the given column id.
    pub fn apply_width(&self, id: &str, width: i32) {
        self.state.column_state.update(|m| {
            let entry = m.entry(id.to_string()).or_insert_with(|| ColumnState {
                id: id.to_string(),
                ..Default::default()
            });
            entry.width = Some(width);
        });
    }

    /// Auto-size a column to fit header text length heuristically (and respect min/max).
    /// This is a simple approximation that multiplies visible header text length by a character width.
    /// For more accuracy, integrate actual text measurement later.
    pub fn auto_size_header_only(&self, id: &str) {
        // Find header text and constraints
        let (header, padding, min_w, max_w_opt) = self.state.columns.with(|cols| {
            if let Some(c) = cols.iter().find(|c| c.id == id) {
                (
                    c.header_name.to_string(),
                    16,
                    c.min_width.max(0),
                    c.max_width,
                )
            } else {
                (String::new(), 16, 0, None)
            }
        });
        // Heuristic: ~8 px per character for default font, plus padding
        let mut w = ((header.len() as i32) * 8) + padding;
        if let Some(maxw) = max_w_opt {
            w = w.min(maxw);
        }
        w = w.max(min_w);
        self.apply_width(id, w);
    }
}
