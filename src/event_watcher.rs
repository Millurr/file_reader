use std::path::Path;

use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Config};

use crate::csv_reader;
use crate::txt_reader;

pub fn watch_folder<P: AsRef<Path>>(path: P) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                // let evnt = event.clone();
                let path = String::from(event.paths[0].to_string_lossy());

                if event.kind.is_modify() {
                    println!("Modify event: {:?}", &path);
                }
                else if event.kind.is_create() {
                    println!("Create event: {:?}", &path);
                }
                else if event.kind.is_access() {
                    publish_event(&path);
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn publish_event(file: &str) {
    if file.contains(".csv") {
        let contents = csv_reader::read_file_from_path(file);
        println!("{:?}", contents);
    }
    else if file.contains(".xlsx") {
        todo!();
    }
    else if file.contains(".txt") {
        let contents = txt_reader::read_file_from_path(file).expect("Error getting content from the file.");
        println!("{}", contents);
    }
}