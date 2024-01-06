use std::fs;
use std::path::Path;
use std::str;
fn read_file(filename: &Path) -> () {
    let data = fs::read(filename).unwrap();
    let string = str::from_utf8(&data).unwrap();
    println!(
        "Data from {}: size:{}\n{} ",
        filename.to_str().unwrap(),
        string.bytes().count(),
        string
    );
}

fn read_dir(dir_path: &Path) {
    if dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    //  println!("{}\n", entry.file_name().into_string().unwrap())
                    read_file(&entry.path());
                }
            }
        }
    }
}
fn main() {
    //    read_file(Path::new("abc.txt"));
    read_dir(Path::new("test_dir"));
}
