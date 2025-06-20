mod library_manager;

use library_manager::LibraryInterface;

fn main() {
    let mut library_interface = LibraryInterface::new();

    loop {
        match library_interface.run_library_user_interface_iteration() {
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
