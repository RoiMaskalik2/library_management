use crate::library_manager::error::Result;
use crate::library_manager::{Library, LibraryError};
use std::io::stdin;
use std::ops::{ControlFlow, RangeInclusive};

const VALID_LIBRARY_CHOICE_RANGE: RangeInclusive<u32> = 1..=8;

/// TODO: Move to another consts file
// This enum represents all of the different choices that the user has when interacting with the library interface
enum LibraryInterfaceChoice {
    AddBookToLibrary,
    RemoveBookFromLibrary,
    AddCopiesToExistingStorage,
    BorrowBook,
    ReturnBook,
    GetBookInformation,
    GetAllBooksInLibraryInformation,
    LeaveInterface,
    InvalidChoice,
}

impl From<u32> for LibraryInterfaceChoice {
    fn from(numeric_choice: u32) -> Self {
        match numeric_choice {
            1 => LibraryInterfaceChoice::AddBookToLibrary,
            2 => LibraryInterfaceChoice::RemoveBookFromLibrary,
            3 => LibraryInterfaceChoice::AddCopiesToExistingStorage,
            4 => LibraryInterfaceChoice::BorrowBook,
            5 => LibraryInterfaceChoice::ReturnBook,
            6 => LibraryInterfaceChoice::GetBookInformation,
            7 => LibraryInterfaceChoice::GetAllBooksInLibraryInformation,
            8 => LibraryInterfaceChoice::LeaveInterface,
            _ => LibraryInterfaceChoice::InvalidChoice,
        }
    }
}

pub struct LibraryInterface {
    library: Library,
}

impl LibraryInterface {
    /// TODO: Document
    pub fn new() -> Self {
        Self {
            library: Library::new(),
        }
    }

    // TODO: Fix documentation
    /// This function runs the main user interface loop for using the self.library.
    ///
    /// # Arguments
    ///
    /// * `library` - A mutable reference to a Library struct,
    /// will be used to store data and make operations related to the self.library.
    pub fn run_library_user_interface_iteration(&mut self) -> Result<ControlFlow<()>> {
        Self::print_welcome_message();

        let library_choice = Self::get_library_choice_from_user()?;
        self.handle_library_choice(library_choice)
    }

    /// Handles a user's choice by calling the appropriate library function.
    ///
    /// # Arguments
    ///
    /// * `library_choice` - a choice from the library's menu.
    /// * `library` - A mutable reference to a Library struct.
    ///
    /// # Returns
    ///
    /// * `ControlFlow::Continue(())` if the program should continue running after handling the cohice.
    /// * `ControlFlow::Break(())` if the user has chosen to exit.
    fn handle_library_choice(&mut self, library_choice: u32) -> Result<ControlFlow<()>> {
        match LibraryInterfaceChoice::from(library_choice) {
            LibraryInterfaceChoice::AddBookToLibrary => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;
                let author_name = Self::read_user_input(String::from("Enter book author name:"))?;

                self.library
                    .create_new_book_storage(book_name, author_name)?;
            }
            LibraryInterfaceChoice::RemoveBookFromLibrary => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.remove_book_storage(book_name)?;
            }
            LibraryInterfaceChoice::AddCopiesToExistingStorage => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;
                let copy_amount = Self::read_user_numeric_input(String::from(
                    "Enter amount of copies you want to add to the storage: ",
                ))?;

                self.library
                    .add_multiple_book_copies(book_name, copy_amount)?;
            }
            LibraryInterfaceChoice::BorrowBook => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.borrow_book(book_name)?;
            }
            LibraryInterfaceChoice::ReturnBook => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.return_book(book_name)?;
            }
            LibraryInterfaceChoice::GetBookInformation => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.print_book(book_name)?;
            }
            LibraryInterfaceChoice::GetAllBooksInLibraryInformation => {
                println!("{}", self.library);
            }
            LibraryInterfaceChoice::LeaveInterface => {
                println!("I will miss you :(");

                return Ok(ControlFlow::Break(()));
            }
            LibraryInterfaceChoice::InvalidChoice => {
                // TODO: How can I handle that???
                println!(
                    "dat number is not in de list, How did you even get here, I already check that.."
                );
            }
        }

        Ok(ControlFlow::Continue(()))
    }

    /// This function prints the library's instruction for the user
    fn print_welcome_message() {
        println!(
            "-----------------------------------------
        Hello Expensive bro, this is library, library can do dis tings:
    1. add a new book to library
    2. make book disapir from library bye bye
    3. Donate to an existing storage!!!!!
    4. want borrow book? no problem
    5. want return book? no problem
    6. want to know who de modefocker that wrote de book? no problem
    7. want to know all secrets of library? no problem
    8. want leave? leave stupid
    -----------------------------------------"
        );
    }

    // --------------------------------- user input parsing functions ------------------------

    /// This function receives a message to display to the user
    /// and returns the user input in a Option<string> format
    ///
    /// # Returns
    ///
    /// The user input if the input is not empty or contains only whitespaces.
    /// None Otherwise
    fn read_user_input(user_message: String) -> Result<String> {
        println!("{}", user_message);

        // Receive input from the user
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)?;

        user_input = user_input.trim().to_string();

        // Validate the user input
        (!user_input.is_empty())
            .then_some(user_input)
            .ok_or(LibraryError::EmptyInputString)
    }

    /// This function receives a message to display to the user
    /// and returns the user input in a Option<string> format
    ///
    /// # Returns
    ///
    /// The user input if the input is a number.
    /// None Otherwise
    fn read_user_numeric_input(user_message: String) -> Result<u32> {
        let user_input: String = Self::read_user_input(user_message)?;

        let numeric_input: u32 = user_input.parse()?;

        Ok(numeric_input)
    }

    /// This function receives a library choice from the user and validates that it a valid library choice
    ///
    /// # Returns
    ///
    /// The input that was received from the user if the library choice is valid (An integer between 1-8)
    /// 'None' Otherwise
    fn get_library_choice_from_user() -> Result<u32> {
        let library_choice =
            Self::read_user_numeric_input(String::from("Enter Interface choice:"))?;

        (VALID_LIBRARY_CHOICE_RANGE)
            .contains(&library_choice)
            .then_some(library_choice)
            .ok_or(LibraryError::UserInputNotInRange)
    }
}
