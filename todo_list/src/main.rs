use std::io;
use clearscreen::clear;
mod crud_todos;
mod crud_json;
mod line_tracker;

fn main() {

    

    let mut user_menu_choice_input = String::new();

    let user_menu_choice = loop {
        user_menu_choice_input.clear();
        println!("Your Rusty Todos");
        println!("Press 1 to create a new Todo");
        println!("Press 2 to get to get a specific Todo");
        io::stdin()
            .read_line(&mut user_menu_choice_input)
            .expect("Failed to read your input");

        let user_menu_choice_input: u8 = match user_menu_choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if user_menu_choice_input < 1 || user_menu_choice_input > 2 {
            clear().expect("Failed to clear screen");
            continue;
        } else {
            clear().expect("Failed to clear screen");
            break user_menu_choice_input;
        }
    };
    if user_menu_choice == 1 {
        let new_todo = crud_todos::create_todo();
        match crud_json::write_json(new_todo) {
            Ok(()) => (),
            Err(_) => print!("Error writing json")
        };
    }
}
