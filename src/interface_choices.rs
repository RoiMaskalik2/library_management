//! This module groups all it's submodules for creating a library user interface which allows to receive choices
//! by the user and convert them into an operation in the library
pub mod book;
pub mod book_storage;
pub mod consts;
pub mod error;
pub mod library;
pub mod library_interface;

pub use book::Book;
pub use book_storage::BookStorage;
pub use error::{LibraryError, Result};
pub use library::Library;
pub use library_interface::LibraryInterface;
