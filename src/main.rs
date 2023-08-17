use std::process;

mod csv_reader;
mod postgres;
mod event_watcher;

fn main() {

    if let Err(err) = csv_reader::read_file_from_path("input/people.csv") {
        println!("error running read_file_from_path: {}", err);
        process::exit(1);
    }

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("Watching {}", path);

    if let Err(error) = event_watcher::notify_change(path) {
        println!("Error: {:?}", error);
    }

    // println!("Start csv file read...");
    // if let Err(err) = csv_reader::read_file_from_arg() {
    //     println!("error running read_file_from_arg: {}", err);
    //     process::exit(1);
    // }

    println!("\nStart postgres data read...");
    if let Err(err) = postgres::query_db() {
        println!("error running query_db: {}", err);
        process::exit(1);
    }
}
