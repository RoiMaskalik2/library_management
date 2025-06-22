//! This crate allows a user to interact with a library interface through the standard input
//! The user can perform multiple choices and receive an indication from the interface about the choice
//! The user interface will continue to run even if there is a bad input from the user.
//! It will only stop when the user will insert the [crate::interface_choices::consts::LibraryInterfaceChoice::LeaveInterface] choice

mod interface_choices;

use interface_choices::LibraryInterface;

fn main() {
    let mut interface_choicess = LibraryInterface::new();

    loop {
        match interface_choicess.run_library_user_interface_iteration() {
            Ok(control_flow) => {
                if control_flow.is_break() {
                    break;
                }
            }
            // In case of an error, we will continue the iteration of the loop
            // In order to make the user try and enter a choice again
            Err(error) => {
                println!("{error}");
            }
        }
    }
}
