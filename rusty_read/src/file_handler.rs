use csv::Reader;

pub fn read_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("CSV/DataFull.csv")?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}