use std::fs;
use std::os::unix;
use std::path;

pub fn remove_all_symlinks(directory_path: &str) {
    for entry in fs::read_dir(directory_path).unwrap() {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let metadata = fs::symlink_metadata(&path).unwrap();
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    println!("Removing symlink");
                    fs::remove_file(&path).unwrap();
                }
            }
            Err(_) => {
                println!("skipping bad dir");
            }
        }
    }
}

pub fn create_symlink(source_path: &str, destination_path: &str) {
    let absolute_src_path = path::absolute(source_path).unwrap();
    let absolute_dst_path = path::absolute(destination_path).unwrap();
    match unix::fs::symlink(absolute_src_path, absolute_dst_path) {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
