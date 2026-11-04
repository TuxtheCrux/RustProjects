use std::io;
mod crud_todos;
mod crud_json;

fn main() {
    println!("Your Rusty Todos");
    println!("Press 1 to create a new Todo");
    println!("Press 2 to get to your Todo-List");

    let mut user_menu_choice_input = String::new();

    let user_menu_choice = loop {
        io::stdin()
            .read_line(&mut user_menu_choice_input)
            .expect("Failed to read your input");

        let user_menu_choice_input: u8 = match user_menu_choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if user_menu_choice_input < 1 || user_menu_choice_input > 2 {
            continue;
        } else {
            break user_menu_choice_input;
        }
    };
    if user_menu_choice == 1 {
        crud_todos::create_todo();
    }
}
