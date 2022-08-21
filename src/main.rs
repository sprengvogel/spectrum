use std::{fs::{self}, env};
use chrono::{DateTime, Local};
use spectrum::{establish_connection, create_file_entry};

fn main() -> std::io::Result<()> {
    let pwd = &env::current_dir()?;
    let files = fs::read_dir(pwd)?;
/*     let filepaths = files.filter_map(|entry| {
        entry.ok().map(|e| e.path())
    })
        .collect::<Vec<PathBuf>>();
     let filedates = filepaths.iter().filter_map(|path| path.metadata().ok()
        .and_then(|m| m.created().ok().and_then(|systime| Some(DateTime::<Local>::from(systime).naive_local()))))
        .collect::<Vec<NaiveDateTime>>();
    println!("{:?}", filepaths);
    println!("{:?}", filedates); */

    let conn = &establish_connection();
    for file_result in files {
        match file_result.and_then(|file| store_file_in_dir(file, conn)) {
            Err(err) => {
                println!("Error {} ignored while storing files.", err);
                continue;
            },
            Ok(()) => continue
        }
    }

    Ok(())
}

fn store_file_in_dir(file: fs::DirEntry, conn: &diesel::SqliteConnection) -> Result<(), std::io::Error> {
    let path = (&file).path();
    let last_modified = DateTime::<Local>::from((&file).metadata()?.created()?).naive_local();
    create_file_entry(conn, (&path).to_str().expect("Filename was not valid utf-8"), &last_modified);
    Ok(())
}
