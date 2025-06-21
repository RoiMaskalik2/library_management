use crate::library_manager::BookStorage;
use crate::library_manager::{LibraryError, Result};

use core::fmt::{Display, Formatter};

use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

    pub fn get_book_storage_by_name(&mut self, book_name: String) -> Result<&mut BookStorage> {
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
    ) -> Result<()> {
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

    /// Removes a book from the library storages, including all of the copies that this book had.
    ///
    /// # Errors
    ///
    /// returns an error if there was no storage create for the book in the library from the first place.
    pub fn remove_book_storage(&mut self, book_name: String) -> Result<()> {
        match self.book_map.entry(book_name) {
            Entry::Vacant(_) => Err(LibraryError::NoBookStorageExist),
            Entry::Occupied(entry) => {
                entry.remove_entry();

                Ok(())
            }
        }
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
    // This consts are used throughout the tests to create a new book storage and access it
    const TEST_BOOK_NAME: &str = "Raifen's Bathroom Experience";
    const TEST_AUTHOR_NAME: &str = "Fuad The Great";

    // This const is used in a test where another book storage was needed to be created
    const TEST_NEW_BOOK_STORAGE_NAME: &str = "Fso Budit Harasho";

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
        let creation_result = library
            .create_new_book_storage(TEST_BOOK_NAME.to_string(), TEST_AUTHOR_NAME.to_string());

        assert!(creation_result.is_ok());
        assert_eq!(library.book_map.len(), 1);

        // Check that the storage was created correctly with one copy
        let storage_result = library.get_book_storage_by_name(TEST_BOOK_NAME.to_string());
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
            .create_new_book_storage(TEST_BOOK_NAME.to_string(), TEST_AUTHOR_NAME.to_string())
            .unwrap();

        // Try to create the same storage again
        assert!(matches!(
            library
                .create_new_book_storage(TEST_BOOK_NAME.to_string(), TEST_AUTHOR_NAME.to_string()),
            Err(LibraryError::CreateExistingBookStorage)
        ));

        // Make sure no book storage was added the second time
        assert_eq!(library.book_map.len(), 1);
    }

    // This test checks that removing a book storage from the library removes all of the books from the library
    #[test]
    fn test_remove_book_storage() {
        let mut library = Library::new();
        library
            .create_new_book_storage(TEST_BOOK_NAME.to_string(), TEST_AUTHOR_NAME.to_string())
            .unwrap();

        // Remove the book storage that was created
        assert!(
            library
                .remove_book_storage(TEST_BOOK_NAME.to_string())
                .is_ok()
        );
        assert!(library.book_map.is_empty());

        // Check that we cannot access it
        assert!(matches!(
            library.get_book_storage_by_name(TEST_BOOK_NAME.to_string()),
            Err(LibraryError::NoBookStorageExist)
        ));
    }

    // This test cheks that removing a not existing test storage is not successful
    #[test]
    fn test_remove_not_existing_book_storage() {
        let mut library = Library::new();

        assert!(matches!(
            library.remove_book_storage(TEST_BOOK_NAME.to_string()),
            Err(LibraryError::NoBookStorageExist)
        ));
    }

    // This test checks that doing operations on one book type does not affect the other
    #[test]
    fn test_different_book_storages() {
        let mut library = Library::new();
        library
            .create_new_book_storage(TEST_BOOK_NAME.to_string(), TEST_AUTHOR_NAME.to_string())
            .unwrap();

        // Create another book storage
        assert!(
            library
                .create_new_book_storage(
                    TEST_NEW_BOOK_STORAGE_NAME.to_string(),
                    TEST_AUTHOR_NAME.to_string()
                )
                .is_ok()
        );

        // Get an initial instance of the new book storage
        let new_book_storage = library
            .get_book_storage_by_name(TEST_NEW_BOOK_STORAGE_NAME.to_string())
            .unwrap()
            .clone();

        // Do Some operations on the first book storage
        let copy_amount: u32 = 10;
        let first_book_storage = library
            .get_book_storage_by_name(TEST_BOOK_NAME.to_string())
            .unwrap();

        first_book_storage.add_multiple_book_copies(copy_amount);

        for _ in 0..=copy_amount {
            assert!(first_book_storage.borrow().is_ok());
        }

        assert!(first_book_storage.return_book().is_ok());

        // Make sure the second book storage did not change
        assert_eq!(
            new_book_storage,
            (*library
                .get_book_storage_by_name(TEST_NEW_BOOK_STORAGE_NAME.to_string())
                .unwrap()),
        )
    }
}
