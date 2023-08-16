use std::process;

mod csv_reader;
mod postgres;

fn main() {
    // if let Err(err) = csv_reader::read_file() {
    //     println!("error running read_file: {}", err);
    //     process::exit(1);
    // }

    if let Err(err) = postgres::query_db() {
        println!("error running query_db: {}", err);
        process::exit(1);
    }
}
