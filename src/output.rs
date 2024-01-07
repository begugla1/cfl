use std::path::PathBuf;

use super::{MAX_ABS_PATH_LEN, MAX_FILE_LEN};

pub fn print_file_len_analisys_result(
    (filename_byte_len, abs_path_len): (usize, usize),
    (filename_len_valid, abs_file_path_len_valid): (bool, bool),
    abs_path: PathBuf,
) {
    println!("Considered file: '{}'", abs_path.display());
    if !(filename_len_valid && abs_file_path_len_valid) {
        if !filename_len_valid {
            println!(
                "Filename len invalid: {}/{} bytes ❌",
                filename_byte_len, MAX_FILE_LEN
            );
        }
        if !abs_file_path_len_valid {
            println!(
                "Absolute file path len invalid: {}/{} bytes ❌",
                abs_path_len, MAX_ABS_PATH_LEN
            );
        }
        println!("You need to change something 🙁")
    } else {
        println!(
            "Filename len valid: {}/{} bytes ✅",
            filename_byte_len, MAX_FILE_LEN
        );
        println!(
            "Absolute file path len valid: {}/{} bytes ✅",
            abs_path_len, MAX_ABS_PATH_LEN
        );
        println!("All right! ✅")
    }
}

pub fn print_doc() {
    println!("Documentation 📚: ");
    println!("'cfl' - is a little program to determine weather it's possible to create/move/copy file/folder in/into
another folder.");
    println!("Create/copy/move <filename> in <dir>: 'cfl <filename> <dest_path>'");
    println!("If given <filename> does exist than (if it's folder) program will try to find the longest path in <filename>
to guarantee that it's possible to move/copy <filename> in <dest_path>");
    println!("P.S: You can avoid passing <dest_path> argument if you suggest the current one");
}
