use std::io::{stdout, Write};

#[derive(Default)]
pub struct Tracker {
    row_count: usize
}


impl Tracker {
    pub fn print(&mut self, message: &str) {
        println!("{}", message);
        self.row_count += message.matches('\n').count() + 1;
    }

    pub fn count_input(&mut self) {
        self.row_count += 1
    }

    pub fn clear(&mut self) {
        if self.row_count > 0 {
            print!(
                "{}{}", termion::cursor::Up(u16::try_from(self.row_count).unwrap_or(u16::MAX)),
                 termion::clear::AfterCursor
            );                
            stdout().flush().expect("Failed to flush stdout");
        }
        self.row_count = 0;
    }
}