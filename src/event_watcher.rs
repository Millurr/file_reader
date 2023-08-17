use std::{path::Path};

use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result, Config, EventKind::Modify, EventKind::Create, EventKind::Access, EventKind::Any, EventKind::Other, EventKind::Remove, event::ModifyKind};

use crate::csv_reader;

pub fn notify_change<P: AsRef<Path>>(path: P) -> Result<()> {
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
                    let _ = csv_reader::read_file_from_path(&path);
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}