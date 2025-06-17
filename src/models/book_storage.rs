//! This module contains implementation for the BookStorage Struct.
//! This struct will be a part of the Library Struct, Each BookStorage has details about
//! the book it contains, the amount of books there are in the storage, and the number of books that are currently borrowed
//! from the book storage

use crate::models::Book;

/// Enum that represents different errors that can occur while handling a BookStorage Struct
pub enum BookStorageError {
    /// Borrowing a book that is not available to borrow
    BorrowWhenNotAvailable,

    /// Returning a book when there are 0 books borrowed from the storage
    NoCopies,
}

/// This struct represents book storage in a library, a book storage contains a book
/// and the amount of copies the book has in the storage.
pub struct BookStorage {
    /// The type of book that is stored in the storage.
    book: Book,

    /// The amount of books from the stored book type that are currenly borrowed.
    borrowed_copy_amount: u32,

    /// The total amount of books there are in the storage (borrowed and unborrowed).
    total_copy_amount: u32,
}

impl BookStorage {
    /// Create a new book storage with a specific amount of of books
    ///
    /// # Arguments
    ///
    /// * `book_name` - the name of the book
    /// * `book_author_name` - the author of the book's name
    /// * `copy_amount` - the amount of copies that there are in the storage
    pub fn new(book_name: String, book_author_name: String, copy_amount: u32) -> Self {
        Self {
            book: Book::new(book_name, book_author_name),
            borrowed_copy_amount: 0,
            total_copy_amount: copy_amount,
        }
    }

    /// Create a new empty book storage.
    /// An empty book storage will contain 0 books.
    ///
    /// # Arguments
    ///
    /// * `book_name` - the name of the book
    /// * `book_author_name` - the author of the book's name
    pub fn new_empty(book_name: String, book_author_name: String) -> Self {
        Self::new(book_name, book_author_name, 0)
    }

    /// This method adds a book copy to an existing book storage
    pub fn add_book_copy(&mut self) {
        self.add_multiple_book_copies(1);
    }

    /// This method adds multiple book copies to an existing book storage
    pub fn add_multiple_book_copies(&mut self, copy_amount: u32) {
        self.total_copy_amount += copy_amount;
    }

    /// Borrows the book if it is available for borrowing
    ///
    /// # Returns
    ///
    /// BorrowWhenNotAvailable when there are no copies to be borrowed, else Ok
    pub fn borrow(&mut self) -> Result<(), BookStorageError> {
        (self.borrowed_copy_amount >= self.total_copy_amount)
            .then_some(())
            .ok_or(BookStorageError::BorrowWhenNotAvailable)?;

        self.borrowed_copy_amount += 1;

        Ok(())
    }

    /// Increases the total amount of available books in the book storage
    ///
    /// # Returns
    ///
    /// NoCopies when there are no copies to be returned, else Ok
    pub fn return_book(&mut self) -> Result<(), BookStorageError> {
        (self.borrowed_copy_amount == 0)
            .then_some(())
            .ok_or(BookStorageError::NoCopies)?;

        self.borrowed_copy_amount -= 1;

        Ok(())
    }
}
