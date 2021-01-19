use std::env;
use std::fs;

fn show_dir(path: &str) -> Result<(), std::io::Error> {
    // read all contents in current_dir if possible
    println!("path length");
    for entry in fs::read_dir(path)? {
        // unwarp path entries and get path one by one
        let path = entry?.path();
        // get metadata from the path if possible
        let md = fs::metadata(path.clone())?;
        // display path and metdata length from the path
        println!("{} {}", path.display(), md.len());
    }
    // return nothing
    Ok(())
}

fn main() {
    match env::args().nth(1) {
        Some(path) => show_dir(&path).expect("couldn't traverse directory"),
        _ => println!("Give a valid path"),
    }
}
