use std::process;

mod csv_reader;
mod txt_reader;
mod postgres;
mod event_watcher;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    let mut i = 0;

    while i < 3 {
        if let Err(error) = event_watcher::watch_folder_trigger(&path) {
            println!("Error: {:?}", error);
        }

        i += 1;
        println!("{}", i);
    }

    println!("Watching {}", path);

    if let Err(error) = event_watcher::watch_folder(path) {
        println!("Error: {:?}", error);
    }

    if let Err(err) = csv_reader::read_file_from_path("input/people.csv") {
        println!("error running read_file_from_path: {}", err);
        process::exit(1);
    }

    println!("\nStart postgres data read...");
    if let Err(err) = postgres::query_db() {
        println!("error running query_db: {}", err);
        process::exit(1);
    }
}
