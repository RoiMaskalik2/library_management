use crate::models::book_storage::{BookStorage, BookStorageError};

use core::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

// Enum that represents different errors that can occur when handling the Library struct
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
