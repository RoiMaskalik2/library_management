use crate::library_manager::consts;
use crate::library_manager::error::Result;
use crate::library_manager::{Library, LibraryError};
use std::io::stdin;
use std::ops::ControlFlow;

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
    fn handle_library_choice(&mut self, library_choice: usize) -> Result<ControlFlow<()>> {
        match consts::LibraryInterfaceChoice::try_from(library_choice)? {
            consts::LibraryInterfaceChoice::AddBookToLibrary => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;
                let author_name = Self::read_user_input(String::from("Enter book author name:"))?;

                self.library
                    .create_new_book_storage(book_name, author_name)?;
            }
            consts::LibraryInterfaceChoice::RemoveBookFromLibrary => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.remove_book_storage(book_name)?;
            }
            consts::LibraryInterfaceChoice::AddCopiesToExistingStorage => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;
                let copy_amount = Self::read_user_numeric_input(String::from(
                    "Enter amount of copies you want to add to the storage: ",
                ))?;

                self.library
                    .get_book_storage_by_name(book_name)?
                    .add_multiple_book_copies(copy_amount);
            }
            consts::LibraryInterfaceChoice::BorrowBook => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library.get_book_storage_by_name(book_name)?.borrow()?;
            }
            consts::LibraryInterfaceChoice::ReturnBook => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                self.library
                    .get_book_storage_by_name(book_name)?
                    .return_book()?;
            }
            consts::LibraryInterfaceChoice::GetBookInformation => {
                let book_name = Self::read_user_input(String::from("Enter book name:"))?;

                let book_storage = self.library.get_book_storage_by_name(book_name)?;
                println!("{}", book_storage)
            }
            consts::LibraryInterfaceChoice::GetAllBooksInLibraryInformation => {
                println!("{}", self.library);
            }
            consts::LibraryInterfaceChoice::LeaveInterface => {
                println!("I will miss you :(");

                return Ok(ControlFlow::Break(()));
            }
        }

        Ok(ControlFlow::Continue(()))
    }

    /// This function prints the library's instruction for the user
    fn print_welcome_message() {
        println!(
            "-----------------------------------------
        Hello Expensive bro, this is library, library can do dis tings :
    {}. add a new book to library
    {}. make book disapir from library bye bye
    {}. Donate to an existing storage!!!!!
    {}. want borrow book? no problem
    {}. want return book? no problem
    {}. want to know who de modefocker that wrote de book? no problem
    {}. want to know all secrets of library? no problem
    {}. want leave? leave stupid
    -----------------------------------------",
            consts::ADD_BOOK_TO_LIBRARY,
            consts::REMOVE_BOOK_FROM_LIBRARY,
            consts::ADD_COPIES_TO_EXISTING_STORAGE,
            consts::BORROW_BOOK,
            consts::RETURN_BOOK,
            consts::GET_BOOK_INFORMATION,
            consts::GET_ALL_BOOKS_IN_LIBRARY_INFORMATION,
            consts::LEAVE_INTERFACE
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
    fn read_user_numeric_input(user_message: String) -> Result<usize> {
        let user_input: String = Self::read_user_input(user_message)?;

        let numeric_input: usize = user_input.parse()?;

        Ok(numeric_input)
    }

    /// This function receives a library choice from the user and validates that it a valid library choice
    ///
    /// # Returns
    ///
    /// The input that was received from the user if the library choice is valid (An integer between 1-8)
    /// 'None' Otherwise
    fn get_library_choice_from_user() -> Result<usize> {
        let library_choice =
            Self::read_user_numeric_input(String::from("Enter Interface choice:"))?;

        (consts::VALID_LIBRARY_CHOICE_RANGE)
            .contains(&library_choice)
            .then_some(library_choice)
            .ok_or(LibraryError::UserInputNotInRange)
    }
}
