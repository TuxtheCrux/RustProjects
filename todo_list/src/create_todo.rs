use chrono::prelude::*;
use std::io;
struct ToDos {
    id: u32,
    title: String,
    creation_date: NaiveDate,
    deadline: Option<NaiveDate>,
    todo: String,
    done: bool,
}

fn create_todo() -> ToDos {
    let mut todo_title: String = String::new();
    let mut todo: String = String::new();
    let mut deadline_input: String = String::new();

    print!("Please enter a title: ");
    io::stdin()
        .read_line(&mut todo_title)
        .expect("Failed to read Input 'todo_title'");
    let todo_title: String = todo_title.trim().to_string();

    print!("\nWhat do you need to do?: ");
    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read input 'todo'");
    let todo: String = todo.trim().to_string();

    let deadline: Option<NaiveDate> = loop {
        deadline_input.clear();
        println!("Please enter the deadline in the format 'dd.mm.yyyy'\n
                If you just hit 'Enter' without a input you will receive no deadline: ");
        io::stdin()
            .read_line(&mut deadline_input)
            .expect("Failed to read input 'deadline'");

        let temp_deadline = deadline_input.trim();

        if temp_deadline.is_empty() {
            break None;
        }
        match NaiveDate::parse_from_str(temp_deadline, "%d.%m.%Y") {
            Ok(date) => break Some(date),
            Err(e) => println!(
                "Please make sure that you entered the date correctly (dd.mm.yyyy)\n{e} "
            ),
        };
    };

    ToDos {
        id:  rand::random(),
        title: todo_title,
        creation_date: Local::now().date_naive(),
        deadline: deadline, 
        todo: todo, 
        done: false
    }
}
