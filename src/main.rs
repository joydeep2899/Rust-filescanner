use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::Instant;

fn hash_data(data: &Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    return hasher.finish();
}

fn read_file(filepath: &Path) -> () {
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

fn read_dir(dir_path: &Path) {
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
                        read_dir(&path);
                    } else {
                        read_file(&path);
                    }
                }
            }
        }
    } else {
        let pathbuf = dir_path.to_str().unwrap();
        println!("Could Not Read Path : {pathbuf}");
    }
}

fn main() {
    let env_args = env::args();
    let mut n_args = env_args.count();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    n_args -= 1;
    let now = Instant::now();
    if n_args > 0 {
        for arg in args {
            read_dir(Path::new(&arg));
        }
    } else {
        println!("pass atleast 1 argument")
    }
    let elapsed_time = now.elapsed();
    println!(
        "Running Scan took {} milli seconds.",
        elapsed_time.as_millis()
    );
}
