use csv::{Error, ReaderBuilder};

pub fn read_file() -> Result<Vec<Vec<String>>, Error> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path("csv/Test.csv")?; //Qualifing data as correct and handing error-handling to caller
    let mut content = Vec::new(); // Vec<Vec<String>> to get seperate lines and columns 
    for result in reader.records() {
        match result {
            Ok(rec) => {
                let row: Vec<String> = rec.iter().map(String::from).collect(); //parsing every row from Stringrecord to string 
                content.push(row);
            }
            Err(e) => eprintln!("{}", e),
        };
    }
    Ok(content)
}
