
use std::{fs, path::PathBuf};
use crate::crud_todos::ToDos;
use dirs::{document_dir, home_dir};

//TODO: Better Error-Handling with precise error messages and Handling
//TODO: Nach Konventionen unten ändern 
/*
    Konventionen
        Result<(), ()> in einer pub fn: cargo clippy warnt davor (result_unit_err). 
        Die Rust-Konvention: Eine Funktion gibt den Fehler zurück, und der Aufrufer entscheidet,
        ob und wie er ihn anzeigt. eprintln! in einer Speicher-Funktion vermischt zwei Aufgaben. 
        Das ist genau dein TODO, für den Lernstand ist es okay so.
        t-odo: ToDos per Wert: Die Funktion liest nur, also &ToDos. 
        Sonst kann der Aufrufer sein Todo nach dem Speichern nicht mehr verwenden.
        Meldungen mit Pfad: „Cannot create dir ‘todos’“ sagt nicht, wo. 
        Mit .display() auf dem Pfad steht in der Meldung, was genau schiefging. 
        Dafür darfst du sub_dir und file nicht per Wert in create_dir_all/write hineingeben (Move), sondern per Referenz.
*/
pub fn write_json(todo: ToDos) -> Result<(),()> {
    let new_json: String =  match serde_json::to_string(&todo) {
        Ok(json) => json,
        Err(_) => return Err(())
    };
    let basis: PathBuf = document_dir().or_else(home_dir).ok_or(())?;
    let sub_dir = basis.join("todos");
    let file_name = format!("{}.json", todo.id);
    let file = sub_dir.join(file_name);
    
    if let Err(e) = fs::create_dir_all(&sub_dir) {
        eprintln!("Cannot create dir 'todos': {e}");
        return Err(())
    } 
    if let Err(e) = fs::write(file, &new_json) {
        eprintln!("Failed to write to file: {e}");
        return Err(())
    }
    Ok(())
}
