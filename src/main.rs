mod models;

use models::Library;
use std::io;
use std::ops::{ControlFlow, RangeInclusive};

const VALID_LIBRARY_CHOICE_RANGE: RangeInclusive<u32> = 1..=8;

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

fn main() {
    let mut library = Library::new();

    run_library_user_interface(&mut library);
}

/// Thus function runs the main user interface loop for using the library.
///
/// # Arguments
///
/// * `library` - A mutable reference to a Library struct,
/// will be used to store data and make operations related to the library.
fn run_library_user_interface(library: &mut Library) {
    loop {
        print_welcome_message();

        if let Some(library_choice) = get_library_choice_from_user() {
            if let ControlFlow::Break(_) = handle_library_choice(library_choice, library) {
                break;
            }
        }
    }
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
fn handle_library_choice(library_choice: u32, library: &mut Library) -> ControlFlow<()> {
    match LibraryInterfaceChoice::from(library_choice) {
        LibraryInterfaceChoice::AddBookToLibrary => {
            if let Some((book_name, author_name)) = get_book_and_author_from_user() {
                if let Err(library_error) = library.create_new_book_storage(book_name, author_name)
                {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::RemoveBookFromLibrary => {
            if let Some(book_name) = read_book_name() {
                if let Err(library_error) = library.remove_book_storage(book_name) {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::AddCopiesToExistingStorage => {
            if let Some((book_name, copy_amount)) = get_book_and_copy_amount_from_user() {
                if let Err(library_error) = library.add_multiple_book_copies(book_name, copy_amount)
                {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::BorrowBook => {
            if let Some(book_name) = read_book_name() {
                if let Err(library_error) = library.borrow_book(book_name) {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::ReturnBook => {
            if let Some(book_name) = read_book_name() {
                if let Err(library_error) = library.return_book(book_name) {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::GetBookInformation => {
            if let Some(book_name) = read_book_name() {
                if let Err(library_error) = library.print_book(book_name) {
                    eprintln!("{library_error}");
                }
            }
        }
        LibraryInterfaceChoice::GetAllBooksInLibraryInformation => {
            println!("{library}");
        }
        LibraryInterfaceChoice::LeaveInterface => {
            println!("I will miss you :(");
            return ControlFlow::Break(());
        }
        LibraryInterfaceChoice::InvalidChoice => {
            println!(
                "dat number is not in de list, How did you even get here, I already check that.."
            );
        }
    }

    ControlFlow::Continue(())
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
fn read_user_input(user_message: String) -> Option<String> {
    println!("{}", user_message);

    // Receive input from the user
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line from standard input, boy your terminal be trippinnn sheesh");

    user_input = user_input.trim().to_string();

    // Validate the user input
    (!user_input.is_empty()).then_some(user_input)
}

/// This function receives a message to display to the user
/// and returns the user input in a Option<string> format
///
/// # Returns
///
/// The user input if the input is a number.
/// None Otherwise
fn read_user_numeric_input(user_message: String) -> Option<u32> {
    let user_input: String = read_user_input(user_message)?;

    match user_input.parse::<u32>() {
        Ok(converted_user_input) => Some(converted_user_input),
        Err(_) => None,
    }
}

/// This function reads a book name from the user and returns it in a Option<string> format
fn read_book_name() -> Option<String> {
    read_user_input(String::from("Enter book name:"))
}

/// This function reads a book author name from the user and returns it in a Option<string> format
fn read_author_name() -> Option<String> {
    read_user_input(String::from("Enter book author name:"))
}

/// This function reads a numeric amount of book copies to add to a book storage and returns it in a Option<u32> format
fn read_copy_amount() -> Option<u32> {
    read_user_numeric_input(String::from(
        "Enter amount of copies you want to add to the storage: ",
    ))
}

/// This function receives a library choice from the user and validates that it a valid library choice
///
/// # Returns
///
/// The input that was received from the user if the library choice is valid (An integer between 1-8)
/// 'None' Otherwise
fn get_library_choice_from_user() -> Option<u32> {
    let library_choice = read_user_numeric_input(String::from("Enter Interface choice:"))?;

    (VALID_LIBRARY_CHOICE_RANGE)
        .contains(&library_choice)
        .then_some(library_choice)
        .or_else(|| {
            println!("The Choice should be in the range on Raifen's IQ (1-8)");
            None
        })
}

/// This function receives a book name and the author name from the user and returns them.
///
/// # Returns
///
/// The input containing the book name and the book author if the input is not empty or contains only whitespaces.
/// None Otherwise
fn get_book_and_author_from_user() -> Option<(String, String)> {
    let book_name = read_book_name()?;
    let author_name = read_author_name()?;

    Some((book_name, author_name))
}

/// This function receives a book name and a numeric amount of copies to add to the book's storage from the user and returns them.
///
/// # Returns
///
/// The input containing the book name and the amount of copies if the input is valid.
/// None Otherwise
fn get_book_and_copy_amount_from_user() -> Option<(String, u32)> {
    let book_name = read_book_name()?;
    let copy_amount = read_copy_amount()?;

    Some((book_name, copy_amount))
}
