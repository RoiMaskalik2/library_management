//! This module contains implementation for the Book Struct.
//! a book has a name and an author
#[derive(Debug)]
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
