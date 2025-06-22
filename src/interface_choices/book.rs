//! This module implements the [Book] Struct.
use std::fmt::{Display, Formatter, Result};

/// This struct represents a book, each book has a name and an author
#[derive(PartialEq, Debug, Clone)]
pub struct Book {
    /// The name of the book
    name: String,

    /// The name of the book author
    author: String,
}

impl Book {
    /// Creates a new instance of a book.
    pub fn new(name: String, author: String) -> Book {
        Book { name, author }
    }

    /// Returns the book's name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the book's author
    pub fn author(&self) -> &String {
        &self.author
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Book Name: {}\nBook Author: {}",
            self.name(),
            self.author()
        )
    }
}
