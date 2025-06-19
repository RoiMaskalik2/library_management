use crate::models::book_storage::{BookStorage, BookStorageError};

use core::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

// Enum that represents different errors that can occur when handling the Library struct
#[derive(Debug, PartialEq)]
pub enum LibraryError {
    // Occurs when creating a new book storage of a book that has already a storage
    CreateExistingBookStorage,

    // Occurs when trying to borrow, return, or add a book to a book storage that does not exist
    NoBookStorageExist,

    // Occurs when there is no available book for borrowing
    NoBooksAvailable,

    // Occurs when there are no books to return - meaning all of the books are in the storage
    NoBooksToReturn,
}

impl From<BookStorageError> for LibraryError {
    fn from(error: BookStorageError) -> Self {
        match error {
            BookStorageError::BorrowWhenNotAvailable => Self::NoBooksAvailable,
            BookStorageError::NoCopies => Self::NoBooksToReturn,
        }
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryError::CreateExistingBookStorage => {
                write!(
                    f,
                    "Expensive Bro, There is already Madaf Sfarim with dis book"
                )
            }
            LibraryError::NoBookStorageExist => {
                write!(f, "You trippin, the is no such book in dis library")
            }
            LibraryError::NoBooksAvailable => {
                write!(f, "Sucker, No books for fuad")
            }
            LibraryError::NoBooksToReturn => {
                write!(
                    f,
                    "Cutiedie Stupidie, You Won't fool me! the storage is full"
                )
            }
        }
    }
}

/// This struct represents a library that contain book storages.
/// Each book storage is represented by the books name.
#[derive(Debug)]
/// Note: We assume that there could not be 2 books that have the same name in a library,
/// even if they have different authors
pub struct Library {
    // my current implementation idea is to have some kind of hash map from a string of a book name
    // to an instance of BookStorage.
    book_map: HashMap<String, BookStorage>,
}

impl Library {
    /// Creates a new empty library
    pub fn new() -> Self {
        Self {
            book_map: HashMap::new(),
        }
    }

    pub fn get_book_storage_by_name(
        &mut self,
        book_name: String,
    ) -> Result<&mut BookStorage, LibraryError> {
        self.book_map
            .get_mut(&book_name)
            .ok_or(LibraryError::NoBookStorageExist)
    }

    /// This method creates a book storage for a new book that has not been in the library yet.
    ///
    /// # Arguments
    ///
    /// * `book_name` - the name of the book
    /// * `book_author_name` - the author of the book's name
    pub fn create_new_book_storage(
        &mut self,
        book_name: String,
        book_author_name: String,
    ) -> Result<(), LibraryError> {
        match self.book_map.entry(book_name) {
            Entry::Occupied(_) => Err(LibraryError::CreateExistingBookStorage),
            Entry::Vacant(entry) => {
                let mut book_storage =
                    BookStorage::new_empty(entry.key().clone(), book_author_name);

                book_storage.add_book_copy();

                entry.insert(book_storage);

                Ok(())
            }
        }
    }

    /// Borrows a copy of a specified book name, if it is available for borrowing.
    /// A book is available for borrowing if there are at least 1 books availble in it's storage.
    ///
    /// # Errors
    ///
    /// returns an error if the book does not exist or if no copies are available to borrow.
    pub fn borrow_book(&mut self, book_name: String) -> Result<(), LibraryError> {
        let storage = self.get_book_storage_by_name(book_name)?;

        storage.borrow().map_err(LibraryError::from)
    }

    /// Returns a borrowed copy of a book.
    ///
    /// # Errors
    ///
    /// Returns an error if the book does not exist or if there are no books to return (book storage is full)
    pub fn return_book(&mut self, book_name: String) -> Result<(), LibraryError> {
        let storage = self.get_book_storage_by_name(book_name)?;

        storage.return_book().map_err(LibraryError::from)
    }

    /// Adds a multiple book copies of an existing book to the library.
    ///
    /// # Errors
    ///
    /// returns an error if there was no storage create for the book in the library.
    pub fn add_multiple_book_copies(
        &mut self,
        book_name: String,
        book_copies: u32,
    ) -> Result<(), LibraryError> {
        let storage = self.get_book_storage_by_name(book_name)?;

        storage.add_multiple_book_copies(book_copies);

        Ok(())
    }

    /// Adds a new copy of an existing book to the library.
    ///
    /// # Errors
    ///
    /// returns an error if there was no storage create for the book in the library.
    pub fn add_book_copy(&mut self, book_name: String) -> Result<(), LibraryError> {
        self.add_multiple_book_copies(book_name, 1)
    }

    /// Removes a book from the library storages, including all of the copies that this book had.
    ///
    /// # Errors
    ///
    /// returns an error if there was no storage create for the book in the library from the first place.
    pub fn remove_book_storage(&mut self, book_name: String) -> Result<(), LibraryError> {
        match self.book_map.entry(book_name) {
            Entry::Vacant(_) => Err(LibraryError::NoBookStorageExist),
            Entry::Occupied(entry) => {
                entry.remove_entry();

                Ok(())
            }
        }
    }

    /// Prints information about the book storage
    ///
    /// # Errors
    ///
    /// Returns an error if the book does not exis
    pub fn print_book(&mut self, book_name: String) -> Result<(), LibraryError> {
        match self.book_map.entry(book_name) {
            Entry::Vacant(_) => Err(LibraryError::NoBookStorageExist),
            Entry::Occupied(mut entry) => {
                println!("{}", entry.get_mut());
                Ok(())
            }
        }
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Library Books:")?;

        if self.book_map.is_empty() {
            return writeln!(f, "The Library is empty");
        }

        for book_storage in self.book_map.values() {
            writeln!(f, "{}", book_storage)?;
            writeln!(f, "----------------------")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_book_name() -> String {
        String::from("Raifen's Bathroom Experience")
    }

    fn test_author_name() -> String {
        String::from("Raifen's Bathroom Experience")
    }

    /// This test checks that creating a new empty library is successful
    #[test]
    fn test_new_library_is_empty() {
        let library = Library::new();
        assert!(library.book_map.is_empty());
    }

    // This test checks that creating a new book storage is successful
    #[test]
    fn test_create_new_book_storage() {
        let mut library = Library::new();
        let creation_result = library.create_new_book_storage(test_book_name(), test_author_name());

        assert!(creation_result.is_ok());
        assert_eq!(library.book_map.len(), 1);

        // Check that the storage was created correctly with one copy
        let storage_result = library.get_book_storage_by_name(test_book_name());
        assert!(storage_result.is_ok());

        let storage = storage_result.unwrap();
        assert_eq!(storage.total_copy_amount(), 1);
        assert_eq!(storage.borrowed_copy_amount(), 0);
    }

    // This test checks that creating the same book storage two times fails
    #[test]
    fn test_create_existing_book_storage() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        // Try to create the same storage again
        assert_eq!(
            library
                .create_new_book_storage(test_book_name(), test_author_name())
                .unwrap_err(),
            LibraryError::CreateExistingBookStorage
        );

        // Make sure no book storage was added the second time
        assert_eq!(library.book_map.len(), 1);
    }

    // This test checks that borrowing a book from the library is successful
    #[test]
    fn test_borrow_book_success() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        assert!(library.borrow_book(test_book_name()).is_ok());

        // Make sure the book was actually borrowed
        let storage = library.get_book_storage_by_name(test_book_name()).unwrap();
        assert_eq!(storage.borrowed_copy_amount(), 1);
    }

    // This test checks that borrowing a not existing book is not successful
    #[test]
    fn test_borrow_not_existing_book() {
        let mut library = Library::new();

        assert_eq!(
            library.borrow_book(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
    }

    // This test checks that borrowing a book when no books are available is not successful
    #[test]
    fn test_borrow_not_available() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        assert!(library.borrow_book(test_book_name()).is_ok());

        // Borrow when all the books are borrowed
        assert_eq!(
            library.borrow_book(test_book_name()),
            Err(LibraryError::NoBooksAvailable)
        );
    }

    // This test checks that returning a book after borrowing one is successful
    #[test]
    fn test_return_book_success() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();
        library.borrow_book(test_book_name()).unwrap();

        // Return a book after borrowing
        assert!(library.return_book(test_book_name()).is_ok());

        // Make sure the book is available for borrowing again
        assert!(library.borrow_book(test_book_name()).is_ok());
    }

    // This test checks that returning a not existing book is not successful
    #[test]
    fn test_return_not_existing_book() {
        let mut library = Library::new();

        assert_eq!(
            library.return_book(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
    }

    // This test checks that returning a book when no book was borrowed is not successful
    #[test]
    fn test_return_fails() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        // Return a book that was never borrowed
        assert_eq!(
            library.return_book(test_book_name()).unwrap_err(),
            LibraryError::NoBooksToReturn
        );
    }

    // This test checks that adding a book copy allows for one more borrow to occur without returning a book
    #[test]
    fn test_add_book_copy() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        assert!(library.add_book_copy(test_book_name()).is_ok());

        // First 2 borrows should be succesful.
        assert!(library.borrow_book(test_book_name()).is_ok());
        assert!(library.borrow_book(test_book_name()).is_ok());

        // Third borrow should fail
        assert_eq!(
            library.borrow_book(test_book_name()).unwrap_err(),
            LibraryError::NoBooksAvailable
        );
    }

    // This test checks that adding multiple book copies allows for multiple borrows
    #[test]
    fn test_add_multiple_book_copies() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        let copy_amount: u32 = 10;

        assert!(
            library
                .add_multiple_book_copies(test_book_name(), copy_amount)
                .is_ok()
        );

        // All of the borrows should be successful
        for _ in 0..=copy_amount {
            assert!(library.borrow_book(test_book_name()).is_ok());
        }

        // This borrow should fail because there are no more books to borrow
        assert_eq!(
            library.borrow_book(test_book_name()).unwrap_err(),
            LibraryError::NoBooksAvailable
        );
    }

    // This test checks that adding a book copy of a not existing book fails
    #[test]
    fn test_add_copy_to_not_existing_book() {
        let mut library = Library::new();

        assert_eq!(
            library.add_book_copy(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
    }

    // This test checks that removing a book storage from the library removes all of the books from the library
    #[test]
    fn test_remove_book_storage() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        // Remove the book storage that was created
        assert!(library.remove_book_storage(test_book_name()).is_ok());
        assert!(library.book_map.is_empty());

        // Every operation on the book name should fail
        assert_eq!(
            library.add_book_copy(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
        assert_eq!(
            library.borrow_book(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
        assert_eq!(
            library.return_book(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
    }

    // This test cheks that removing a not existing test storage is not successful
    #[test]
    fn test_remove_not_existing_book_storage() {
        let mut library = Library::new();

        assert_eq!(
            library.remove_book_storage(test_book_name()).unwrap_err(),
            LibraryError::NoBookStorageExist
        );
    }

    // This test checks that doing operations on one book type does not affect the other
    #[test]
    fn test_different_book_storages() {
        let mut library = Library::new();
        library
            .create_new_book_storage(test_book_name(), test_author_name())
            .unwrap();

        // Create another book storage
        assert!(
            library
                .create_new_book_storage(String::from("Fso Budit Harasho"), test_author_name())
                .is_ok()
        );

        // Get an initial instance of the new book storage
        let new_book_storage = library
            .get_book_storage_by_name(String::from("Fso Budit Harasho"))
            .unwrap()
            .clone();

        // Do Some operations on the first book storage
        let copy_amount: u32 = 10;

        assert!(
            library
                .add_multiple_book_copies(test_book_name(), copy_amount)
                .is_ok()
        );

        for _ in 0..=copy_amount {
            assert!(library.borrow_book(test_book_name()).is_ok());
        }

        assert!(library.return_book(test_book_name()).is_ok());

        // Make sure the second book storage did not change
        assert_eq!(
            new_book_storage,
            (*library
                .get_book_storage_by_name(String::from("Fso Budit Harasho"))
                .unwrap()),
        )
    }
}
