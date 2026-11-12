use chrono::prelude::*;
use std::io;
use serde::{Deserialize, Serialize};
use crate::line_tracker::Tracker;

#[derive(Serialize, Deserialize)]
pub struct ToDos {
    pub id: u32,
    title: String,
    creation_date: NaiveDate,
    deadline: Option<NaiveDate>,
    todo: String,
    done: bool,
}

pub fn create_todo() -> ToDos {

    let mut todo_title: String = String::new();
    let mut todo: String = String::new();
    let mut deadline_input: String = String::new();
    let mut tracker = Tracker::default();

    println!("Please enter a title: ");
    io::stdin()
        .read_line(&mut todo_title)
        .expect("Failed to read Input 'todo_title'\n");
    let todo_title: String = todo_title.trim().to_string();
    

    println!("\nWhat do you need to do?:");
    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read input 'todo'\n");
    let todo: String = todo.trim().to_string();

    /*---Deadline---*/
    let deadline: Option<NaiveDate> = loop {
        deadline_input.clear();
        tracker.print(
            "Please enter the deadline in the format 'dd.mm.yyyy'\nIf you just hit 'Enter' without a input you will receive no deadline: "
        );
        io::stdin()
            .read_line(&mut deadline_input)
            .expect("Failed to read input 'deadline'\n");
        tracker.count_input();
        let temp_deadline = deadline_input.trim();

        if temp_deadline.is_empty() {
            break None;
        }
        match NaiveDate::parse_from_str(temp_deadline, "%d.%m.%Y") {
            Ok(date) => break Some(date),
            Err(e) => {
                let error_message = format!(
                    "Please make sure that you entered the date correctly (dd.mm.yyyy)\n{e}"
                );
                tracker.print(&error_message);
                tracker.clear();
            },
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


pub fn load_todo(todo: ToDos) {
    println!("--- {} - {} ---", todo.title, todo.creation_date);
    match todo.deadline {
        Some(date) => println!("--- {} - {} ---", todo.done, date),
        None => println!("--- {} - None ---", todo.done),
    };
    println!("---------------");
    println!("{}", todo.todo);
}