pub mod book;
pub mod book_storage;
pub mod error;
pub mod library;
pub mod library_interface;

pub use book::Book;
pub use book_storage::BookStorage;
pub use error::{LibraryError, Result};
pub use library::Library;
pub use library_interface::LibraryInterface;
