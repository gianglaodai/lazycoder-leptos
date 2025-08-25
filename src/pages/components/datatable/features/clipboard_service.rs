pub struct ClipboardService;

impl ClipboardService {
    /// Create a new clipboard service handle.
    pub fn new() -> Self {
        Self
    }

    /// Copy the current selection to the OS clipboard.
    ///
    /// Note: selection integration is not wired yet, so this performs a safe no-op.
    /// When row/column selection is implemented, this should gather the selected
    /// cells/rows and compose a delimited string before copying.
    pub fn copy_selection(&self) {
        // For now, perform a best-effort clipboard write of an empty string in WASM targets,
        // and otherwise a no-op on non-WASM platforms. Ignore errors to avoid panics.
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = leptos::web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                // Fire and forget; we can't block on the returned Promise here.
                let _ = clipboard.write_text("");
                return;
            }
            // If Clipboard API is not available, do nothing for now.
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // No-op on non-wasm targets (e.g., server-side rendering)
        }
    }

    /// Attempt to paste text from the OS clipboard.
    /// Returns None for now as pasting requires async access in WASM; left as a no-op.
    pub fn paste(&self) {
        // Intentionally a no-op. In wasm, reading from clipboard is async (returns a Promise),
        // so this API should be redesigned to be async or to accept a callback.
        // Keeping it non-panicking for now.
    }
}
