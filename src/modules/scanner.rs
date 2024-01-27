use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;


/// func to calculate hash of Data 
fn hash_data(data: &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    return hasher.finish();
}

/// func to scan file 
fn scan_file(filepath: &Path) -> () {
    let data = fs::read(filepath).unwrap();
    if let Ok(metadata) = filepath.metadata() {
        let hash = hash_data(&data);

        println!(
            "\n\nFile Name : {} \nFull Path: {} \nFile Size:{}",
            filepath.to_str().unwrap(),
            fs::canonicalize(filepath).unwrap().display(),
            metadata.len()
        );
        println!("Hash is {:}", hash);
    }
}

/// func to scan dir
pub fn scan(dir_path: &Path) {
    if dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(metadata) = path.metadata() {
                            println!(
                                "Dir Name : {} Dir Size:{}",
                                path.to_str().unwrap(),
                                metadata.len()
                            )
                        }
                        scan(&path);
                    } else {
                        scan_file(&path);
                    }
                }
            }
        }
    } else {
        let pathbuf = dir_path.to_str().unwrap();
        println!("Could Not Read Path : {pathbuf}");
    }
}

