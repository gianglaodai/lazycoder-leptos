pub mod button;
pub mod dialog;
pub mod footer;
pub mod input;
pub mod markdown_editor;
pub mod markdown_viewer;
pub mod navigation;
pub mod pagination;
mod paginator;
pub mod select;
pub mod table;
pub mod textarea;

// Re-export components for easier importing
pub use button::Button;
pub use dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogOverlay, DialogTitle, DialogTrigger,
};
pub use footer::Footer;
pub use input::Input;
pub use markdown_editor::MarkdownEditor;
pub use markdown_viewer::MarkdownViewer;
pub use navigation::Navigation;
pub use pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
pub use paginator::Paginator;
pub use select::Select;
pub use table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
pub use textarea::Textarea;
