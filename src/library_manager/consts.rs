use crate::library_manager::error::LibraryError;
use std::ops::RangeInclusive;

/// Represents the range of all of the valid library choices in the library user interface
pub const VALID_LIBRARY_CHOICE_RANGE: RangeInclusive<usize> = 1..=8;

/// Represents a library user interface choice to add a new book storage to the library
pub const ADD_BOOK_TO_LIBRARY: usize = 1;

/// Represents a library user interface choice to add an existing book storage from the library
pub const REMOVE_BOOK_FROM_LIBRARY: usize = 2;

/// Represents a library user interface choice to add more copies to an existing book storage
pub const ADD_COPIES_TO_EXISTING_STORAGE: usize = 3;

/// Represents a library user interface choice to borrow a book from an existing book storage in the library
pub const BORROW_BOOK: usize = 4;

/// Represents a library user interface choice to return a borrowed book to the library
pub const RETURN_BOOK: usize = 5;

/// Represents a library user interface choice to get information about a specific book in the library
pub const GET_BOOK_INFORMATION: usize = 6;

/// Represents a library user interface choice to get information about all of the books in the library
pub const GET_ALL_BOOKS_IN_LIBRARY_INFORMATION: usize = 7;

/// Represents a library user interface choice to leave the library interface
pub const LEAVE_INTERFACE: usize = 8;

/// This enum represents all of the different choices that the user has when interacting with the library interface
pub enum LibraryInterfaceChoice {
    AddBookToLibrary,
    RemoveBookFromLibrary,
    AddCopiesToExistingStorage,
    BorrowBook,
    ReturnBook,
    GetBookInformation,
    GetAllBooksInLibraryInformation,
    LeaveInterface,
}

impl TryFrom<usize> for LibraryInterfaceChoice {
    type Error = LibraryError;

    fn try_from(numeric_choice: usize) -> Result<Self, Self::Error> {
        match numeric_choice {
            ADD_BOOK_TO_LIBRARY => Ok(LibraryInterfaceChoice::AddBookToLibrary),
            REMOVE_BOOK_FROM_LIBRARY => Ok(LibraryInterfaceChoice::RemoveBookFromLibrary),
            ADD_COPIES_TO_EXISTING_STORAGE => {
                Ok(LibraryInterfaceChoice::AddCopiesToExistingStorage)
            }
            BORROW_BOOK => Ok(LibraryInterfaceChoice::BorrowBook),
            RETURN_BOOK => Ok(LibraryInterfaceChoice::ReturnBook),
            GET_BOOK_INFORMATION => Ok(LibraryInterfaceChoice::GetBookInformation),
            GET_ALL_BOOKS_IN_LIBRARY_INFORMATION => {
                Ok(LibraryInterfaceChoice::GetAllBooksInLibraryInformation)
            }
            LEAVE_INTERFACE => Ok(LibraryInterfaceChoice::LeaveInterface),
            _ => Err(LibraryError::InvalidInterfaceChoiceConversion),
        }
    }
}
