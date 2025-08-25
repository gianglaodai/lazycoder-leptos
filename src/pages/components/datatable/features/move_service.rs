#[derive(Clone, Debug, Default)]
pub struct MoveService {
    dragging: bool,
    col_id: Option<String>,
    start_x: i32,
    current_x: i32,
}

impl MoveService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a drag operation for a column.
    /// col_id is stored for reference; actual reordering is handled elsewhere.
    pub fn begin_drag(&mut self, col_id: &str, start_x: i32) {
        self.dragging = true;
        self.col_id = Some(col_id.to_string());
        self.start_x = start_x;
        self.current_x = start_x;
    }

    /// Update current pointer X during drag. Safe no-op if not dragging.
    pub fn update_drag(&mut self, current_x: i32) {
        if self.dragging {
            self.current_x = current_x;
        }
    }

    /// Drop at a target index. This service does not mutate columns directly; it only
    /// resets its internal drag state. The caller should perform the actual reorder
    /// using ColumnApi::move_column(from, to) or similar when wiring events.
    pub fn drop_at(&mut self, _target_index: usize) {
        // reset state
        self.dragging = false;
        self.col_id = None;
        self.start_x = 0;
        self.current_x = 0;
    }

    /// Returns whether a drag is in progress.
    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    /// Returns the current horizontal delta since begin_drag.
    pub fn delta_x(&self) -> i32 {
        if self.dragging {
            self.current_x - self.start_x
        } else {
            0
        }
    }

    /// Returns the id of the column being dragged, if any.
    pub fn dragging_col_id(&self) -> Option<&str> {
        self.col_id.as_deref()
    }
}
