pub mod footer;
pub mod markdown_editor;
pub mod markdown_viewer;
pub mod navigation;
pub mod button;
pub mod input;
pub mod select;
pub mod textarea;
pub mod dialog;
pub mod table;
pub mod pagination;

// Re-export components for easier importing
pub use button::Button;
pub use footer::Footer;
pub use markdown_editor::MarkdownEditor;
pub use markdown_viewer::MarkdownViewer;
pub use navigation::Navigation;
pub use input::Input;
pub use select::Select;
pub use textarea::Textarea;
pub use dialog::{Dialog, DialogTrigger, DialogOverlay, DialogContent, DialogHeader, DialogFooter, DialogTitle, DialogDescription, DialogClose};
pub use table::{Table, TableHeader, TableBody, TableFooter, TableRow, TableHead, TableCell, TableCaption};
pub use pagination::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
