//! This module contains implementation for the BookStorage Struct.
//! This struct will be a part of the Library Struct, Each BookStorage has details about
//! the book it contains, the amount of books there are in the storage, and the number of books that are currently borrowed
//! from the book storage

use crate::library_manager::Book;
use crate::library_manager::error::{LibraryError, Result};
use ::std::fmt;

/// This struct represents book storage in a library, a book storage contains a book
/// and the amount of copies the book has in the storage.
#[derive(PartialEq, Debug, Clone)]
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

    /// Returns the total book copy amount that are in the storage
    pub fn total_copy_amount(&self) -> u32 {
        self.total_copy_amount
    }

    /// Returns the amount of copies that are currently borrowed from the storage
    pub fn borrowed_copy_amount(&self) -> u32 {
        self.borrowed_copy_amount
    }

    /// Returns a reference to the Book struct that contains book details of the book storage
    pub fn book(&self) -> &Book {
        &self.book
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
    pub fn borrow(&mut self) -> Result<()> {
        (self.borrowed_copy_amount < self.total_copy_amount)
            .then_some(())
            .ok_or(LibraryError::NoBooksAvailable)?;

        self.borrowed_copy_amount += 1;

        Ok(())
    }

    /// Increases the total amount of available books in the book storage
    ///
    /// # Returns
    ///
    /// NoCopiesToReturn when there are no copies to be returned, else Ok
    pub fn return_book(&mut self) -> Result<()> {
        (self.borrowed_copy_amount > 0)
            .then_some(())
            .ok_or(LibraryError::NoCopiesToReturn)?;

        self.borrowed_copy_amount -= 1;

        Ok(())
    }
}

impl fmt::Display for BookStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.book())?;
        write!(
            f,
            "{} Books Borrowed Out Of {} Total Books",
            self.borrowed_copy_amount(),
            self.total_copy_amount()
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // This function creates a default book storage strcut for the tests used in this file
//     fn create_test_storage(total_copies: u32) -> BookStorage {
//         BookStorage::new(
//             String::from("Fuad The Greatest Guy"),
//             String::from("Raifen The Sigma"),
//             total_copies,
//         )
//     }

//     // This test checks the `new` method
//     #[test]
//     fn test_initialize() {
//         let storage = create_test_storage(5);
//         assert_eq!(storage.book().name(), "Fuad The Greatest Guy");
//         assert_eq!(storage.book().author(), "Raifen The Sigma");
//         assert_eq!(storage.total_copy_amount, 5);
//         assert_eq!(storage.borrowed_copy_amount, 0);
//     }

//     // This test checks that when creating a new empty book storage it has zero copies of books in it
//     #[test]
//     fn test_new_empty_book_storage() {
//         let empty_storage = BookStorage::new_empty(
//             String::from("Expensive Brother"),
//             String::from("Fuadini Bombini"),
//         );

//         assert_eq!(empty_storage.total_copy_amount, 0);
//         assert_eq!(empty_storage.borrowed_copy_amount, 0);
//     }

//     // This test checks that adding a book to the storage increases only the totatl copies and not the borrowed copies
//     #[test]
//     fn test_add_book_copy() {
//         let mut storage = create_test_storage(3);
//         storage.add_book_copy();
//         assert_eq!(storage.total_copy_amount(), 4);
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//     }

//     // This test checks that adding multiple book copies increase the total amount of copies by the correct amount
//     #[test]
//     fn test_add_multiple_book_copies() {
//         let mut storage = create_test_storage(3);
//         storage.add_multiple_book_copies(5);
//         assert_eq!(storage.total_copy_amount(), 8);
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//     }

//     // This test checks that adding 0 copies does not change to total amount of copies
//     #[test]
//     fn test_add_zero_book_copies() {
//         let mut storage = create_test_storage(3);
//         storage.add_multiple_book_copies(0);
//         assert_eq!(storage.total_copy_amount(), 3);
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//     }

//     // This test checks that borrowing a book is successful when there are available books to borrow from the storage
//     #[test]
//     fn test_borrow_success() {
//         let mut storage = create_test_storage(2);
//         assert_eq!(storage.borrowed_copy_amount, 0);

//         // First borrow
//         assert!(storage.borrow().is_ok());
//         assert_eq!(storage.borrowed_copy_amount(), 1);
//         assert_eq!(storage.total_copy_amount(), 2); // total amount should not change

//         // Second borrow
//         assert!(storage.borrow().is_ok());
//         assert_eq!(storage.borrowed_copy_amount(), 2);
//         assert_eq!(storage.total_copy_amount(), 2); // total amount should not change
//     }

//     // This test checks that borrowing a book is not successful when there are no books left to borrow
//     #[test]
//     fn test_borrow_fails() {
//         let mut storage = create_test_storage(1);
//         let _ = storage.borrow();

//         // Borrow when there all the books are borrowed
//         assert_eq!(
//             storage.borrow().unwrap_err(),
//             LibraryError::NoBooksAvailable,
//         );

//         // Make sure nothing changed after the failed borrow attempt
//         assert_eq!(storage.borrowed_copy_amount(), 1);
//         assert_eq!(storage.total_copy_amount(), 1);
//     }

//     /// This test checks that returning a book is successful after a copy is borrowed
//     #[test]
//     fn test_return_succees() {
//         let mut storage = create_test_storage(3);

//         // Borrow books in order to be able to return them
//         let _ = storage.borrow();
//         let _ = storage.borrow();

//         // First return
//         assert!(storage.return_book().is_ok());
//         assert_eq!(storage.borrowed_copy_amount(), 1);
//         assert_eq!(storage.total_copy_amount(), 3);

//         // Second return
//         assert!(storage.return_book().is_ok());
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//         assert_eq!(storage.total_copy_amount(), 3);
//     }

//     // This test checks that returning a book when no book was borrowed is not successful
//     #[test]
//     fn test_return_fails() {
//         let mut storage = create_test_storage(3);

//         // Return a book that was never borrowed
//         assert_eq!(
//             storage.return_book().unwrap_err(),
//             LibraryError::NoCopiesToReturn
//         );

//         // Make sure nothing changed
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//         assert_eq!(storage.total_copy_amount(), 3);
//     }

//     // This test checks that after a failed borrow, it will be successful after a book return
//     #[test]
//     fn test_borrow_success_after_return() {
//         let mut storage = create_test_storage(1);

//         // Try to borrow 2 books
//         assert!(storage.borrow().is_ok()); // Should be successful
//         assert!(storage.borrow().is_err()); // Shoule fail

//         // Return a book
//         assert!(storage.return_book().is_ok());

//         // Try to borrow again
//         assert!(storage.borrow().is_ok()); // Should be successful
//     }
// }
