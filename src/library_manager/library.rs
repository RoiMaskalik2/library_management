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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test_book_name() -> String {
//         String::from("Raifen's Bathroom Experience")
//     }

//     fn test_author_name() -> String {
//         String::from("Fuad The Great")
//     }

//     /// This test checks that creating a new empty library is successful
//     #[test]
//     fn test_new_library_is_empty() {
//         let library = Library::new();
//         assert!(library.book_map.is_empty());
//     }

//     // This test checks that creating a new book storage is successful
//     #[test]
//     fn test_create_new_book_storage() {
//         let mut library = Library::new();
//         let creation_result = library.create_new_book_storage(test_book_name(), test_author_name());

//         assert!(creation_result.is_ok());
//         assert_eq!(library.book_map.len(), 1);

//         // Check that the storage was created correctly with one copy
//         let storage_result = library.get_book_storage_by_name(test_book_name());
//         assert!(storage_result.is_ok());

//         let storage = storage_result.unwrap();
//         assert_eq!(storage.total_copy_amount(), 1);
//         assert_eq!(storage.borrowed_copy_amount(), 0);
//     }

//     // This test checks that creating the same book storage two times fails
//     #[test]
//     fn test_create_existing_book_storage() {
//         let mut library = Library::new();
//         library
//             .create_new_book_storage(test_book_name(), test_author_name())
//             .unwrap();

//         // Try to create the same storage again
//         assert_eq!(
//             library
//                 .create_new_book_storage(test_book_name(), test_author_name())
//                 .unwrap_err(),
//             LibraryError::CreateExistingBookStorage
//         );

//         // Make sure no book storage was added the second time
//         assert_eq!(library.book_map.len(), 1);
//     }

//     // This test checks that removing a book storage from the library removes all of the books from the library
//     #[test]
//     fn test_remove_book_storage() {
//         let mut library = Library::new();
//         library
//             .create_new_book_storage(test_book_name(), test_author_name())
//             .unwrap();

//         // Remove the book storage that was created
//         assert!(library.remove_book_storage(test_book_name()).is_ok());
//         assert!(library.book_map.is_empty());

//         // Every operation on the book name should fail
//         assert_eq!(
//             library.add_book_copy(test_book_name()).unwrap_err(),
//             LibraryError::NoBookStorageExist
//         );
//         assert_eq!(
//             library.borrow_book(test_book_name()).unwrap_err(),
//             LibraryError::NoBookStorageExist
//         );
//         assert_eq!(
//             library.return_book(test_book_name()).unwrap_err(),
//             LibraryError::NoBookStorageExist
//         );
//     }

//     // This test cheks that removing a not existing test storage is not successful
//     #[test]
//     fn test_remove_not_existing_book_storage() {
//         let mut library = Library::new();

//         assert_eq!(
//             library.remove_book_storage(test_book_name()).unwrap_err(),
//             LibraryError::NoBookStorageExist
//         );
//     }

//     // This test checks that doing operations on one book type does not affect the other
//     #[test]
//     fn test_different_book_storages() {
//         let mut library = Library::new();
//         library
//             .create_new_book_storage(test_book_name(), test_author_name())
//             .unwrap();

//         // Create another book storage
//         assert!(
//             library
//                 .create_new_book_storage(String::from("Fso Budit Harasho"), test_author_name())
//                 .is_ok()
//         );

//         // Get an initial instance of the new book storage
//         let new_book_storage = library
//             .get_book_storage_by_name(String::from("Fso Budit Harasho"))
//             .unwrap()
//             .clone();

//         // Do Some operations on the first book storage
//         let copy_amount: u32 = 10;

//         assert!(
//             library
//                 .add_multiple_book_copies(test_book_name(), copy_amount)
//                 .is_ok()
//         );

//         for _ in 0..=copy_amount {
//             assert!(library.borrow_book(test_book_name()).is_ok());
//         }

//         assert!(library.return_book(test_book_name()).is_ok());

//         // Make sure the second book storage did not change
//         assert_eq!(
//             new_book_storage,
//             (*library
//                 .get_book_storage_by_name(String::from("Fso Budit Harasho"))
//                 .unwrap()),
//         )
//     }
// }
