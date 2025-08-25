#[derive(Clone, Copy, Debug, Default)]
pub struct FocusPos {
    pub row: i32,
    pub col: i32,
}

#[derive(Clone, Debug)]
pub struct KeyboardService {
    focus: FocusPos,
    max_rows: Option<i32>,
    max_cols: Option<i32>,
}

impl KeyboardService {
    pub fn new() -> Self {
        Self {
            focus: FocusPos::default(),
            max_rows: None,
            max_cols: None,
        }
    }

    /// Optionally configure bounds for focus navigation.
    pub fn with_bounds(mut self, max_rows: Option<i32>, max_cols: Option<i32>) -> Self {
        self.max_rows = max_rows;
        self.max_cols = max_cols;
        // Clamp current focus to the new bounds
        self.focus = Self::clamp(self.focus, self.max_rows, self.max_cols);
        self
    }

    pub fn on_key_down(&mut self, key: &str) {
        // default page step for PageUp/PageDown
        const PAGE_STEP: i32 = 10;
        let mut pos = self.focus;
        match key {
            // Arrows
            "ArrowUp" => {
                pos.row = pos.row.saturating_sub(1);
            }
            "ArrowDown" => {
                pos.row = pos.row.saturating_add(1);
            }
            "ArrowLeft" => {
                pos.col = pos.col.saturating_sub(1);
            }
            "ArrowRight" => {
                pos.col = pos.col.saturating_add(1);
            }
            // Home/End operate on columns
            "Home" => {
                pos.col = 0;
            }
            "End" => {
                if let Some(maxc) = self.max_cols {
                    pos.col = (maxc - 1).max(0);
                }
            }
            // PageUp/PageDown operate on rows
            "PageUp" => {
                pos.row = pos.row.saturating_sub(PAGE_STEP);
            }
            "PageDown" => {
                pos.row = pos.row.saturating_add(PAGE_STEP);
            }
            // Tab advances column, wrapping to next row when max_cols is known
            "Tab" => match self.max_cols {
                Some(maxc) if maxc > 0 => {
                    let next = pos.col + 1;
                    if next >= maxc {
                        pos.col = 0;
                        pos.row = pos.row.saturating_add(1);
                    } else {
                        pos.col = next;
                    }
                }
                _ => {
                    pos.col = pos.col.saturating_add(1);
                }
            },
            // Enter moves down a row
            "Enter" => {
                pos.row = pos.row.saturating_add(1);
            }
            _ => {
                // ignore other keys
            }
        }
        // Clamp to bounds and apply
        self.focus = Self::clamp(pos, self.max_rows, self.max_cols);
    }

    pub fn set_focus(&mut self, pos: FocusPos) {
        self.focus = Self::clamp(pos, self.max_rows, self.max_cols);
    }

    pub fn get_focus(&self) -> FocusPos {
        self.focus
    }

    fn clamp(mut pos: FocusPos, max_rows: Option<i32>, max_cols: Option<i32>) -> FocusPos {
        // lower bounds at 0
        if pos.row < 0 {
            pos.row = 0;
        }
        if pos.col < 0 {
            pos.col = 0;
        }
        // upper bounds when known
        if let Some(mr) = max_rows {
            if mr > 0 {
                pos.row = pos.row.min(mr - 1);
            } else {
                pos.row = 0;
            }
        }
        if let Some(mc) = max_cols {
            if mc > 0 {
                pos.col = pos.col.min(mc - 1);
            } else {
                pos.col = 0;
            }
        }
        pos
    }
}
