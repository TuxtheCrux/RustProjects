use std::env;
mod file_handler;
fn main() {
    let args: Vec<String> = env::args().collect();
    let content = file_handler::read_file().unwrap();
    for line in content {
        println!("{}", line.join(",")); //Vec<String> cannot be displayed without joing each cell
    }
}
