use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

///get magic number from a file
fn file_magic(data: &Vec<u8>) -> Vec<u8> {
    let magic: Vec<u8> = vec![data[0], data[1], data[2], data[3]];
    return magic;
}

/// func to calculate hash of Data
fn hash_data(data: &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    print!("file magic :{:#x?}", file_magic(data));
    return hasher.finish();
}

/// func to scan file
fn scan_file(filepath: &Path) -> () {
    let mut data: Vec<u8> = vec![];
    match fs::read(filepath) {
        Ok(d) => {
            data = d;
        }
        Err(err) => {
            print!("Error reading file {}:{}", filepath.display(), err)
        }
    };

    if let Ok(metadata) = filepath.metadata() {
        let hash = hash_data(&data);

        println!(
            "\n\nFile Name : {} \nFull Path: {} \nFile Size:{}",
            filepath.to_str().unwrap(),
            fs::canonicalize(filepath).unwrap().display(),
            metadata.len()
        );
        println!("Hash is {:}", hash);
    } else {
        print!("Error");
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
        scan_file(dir_path);
    }
}
