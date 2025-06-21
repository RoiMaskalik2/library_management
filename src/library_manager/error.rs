use crate::library_manager::consts;
use core::result;
use thiserror::Error;

// Enum that represents different errors that can occur when using the library_manager module
#[derive(Debug, Error)]
pub enum LibraryError {
    // Occurs when creating a new book storage of a book that has already a storage
    #[error("Expensive Bro, There is already Madaf Sfarim with dis book")]
    CreateExistingBookStorage,

    // Occurs when trying to borrow, return, or add a book to a book storage that does not exist
    #[error("You trippin, the is no such book in dis library")]
    NoBookStorageExist,

    // Occurs when there is no available book for borrowing
    #[error("Sucker, No books for fuad")]
    NoBooksAvailable,

    // Occurs when there are no book copy to return - meaning all of the books are in the storage
    #[error("You won't fool me! all of the copies are in the library storage")]
    NoCopiesToReturn,

    // Occurs when a user enters an empty string in the library interface input
    #[error("Don't Give me empty strings boy")]
    EmptyInputString,

    // Occurs when the user tries to access an interface option that is not a valid number
    #[error("The Choice should be in the range on Raifen's IQ ({}-{})", consts::VALID_LIBRARY_CHOICE_RANGE.start(), consts::VALID_LIBRARY_CHOICE_RANGE.end())]
    // TODO: Incorperate the consts file into here
    UserInputNotInRange,

    // Occurs when something unexpected happens when reading input from the terminal
    #[error("Failed to read line from standard input, boy your terminal be trippinnn sheesh")]
    ReadLineError(#[from] std::io::Error),

    // Occurs when failing to convert between a string to an integer
    #[error("You stupid, I asked for number you gib no number...")]
    ParseIntError(#[from] std::num::ParseIntError),

    // Occurs when a converting between an invalid number to a library Interface choice Enum
    #[error("Cannot convert between the given number to a LibraryInterfaceChoice enum")]
    InvalidInterfaceChoiceConversion,
}

pub type Result<T> = result::Result<T, LibraryError>;
