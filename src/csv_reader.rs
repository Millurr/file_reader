use std::{error::Error, io};

#[derive(Debug, serde::Deserialize)]
struct Record {
    first_name: String,
    last_name: String,
    age: i8,
    sex: String
}

pub fn read_file() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.deserialize(){
        let record: Record = result?;
        // println!{"{:?}", record}
        println!("First Name: {}, Last Name: {}, Age: {}, Sex: {}", record.first_name, record.last_name, record.age, record.sex);
    }
    Ok(())
}