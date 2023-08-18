use std::error::Error;

// #[derive(Debug, serde::Deserialize)]
// struct Record {
//     first_name: String,
//     last_name: String,
//     age: i8,
//     sex: String
// }

// pub fn read_file_from_arg() -> Result<(), Box<dyn Error>> {
//     let mut reader = csv::Reader::from_reader(io::stdin());
//     for result in reader.deserialize(){
//         let record: Record = result?;
//         // println!{"{:?}", record}
//         println!("First Name: {}, Last Name: {}, Age: {}, Sex: {}", record.first_name, record.last_name, record.age, record.sex);
//     }
//     Ok(())
// }

pub fn read_file_from_path(path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut res: Vec<(String, String)> = Vec::new();
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers()?;
    let hdrs = headers.clone();

    for result in reader.records() {
        let record = result?;

        for i in 0..hdrs.len() {
            // println!("{:?}: {:?}", &hdrs[i].to_string(), &record[i].to_string());
            res.push((hdrs[i].to_string(), record[i].to_string()))
        }
    }

    Ok(res)
}