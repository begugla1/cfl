use super::{MAX_ABS_PATH_LEN, MAX_FILE_LEN, RESULT_EMOJII};

pub fn print_file_len_analisys_result(
    (filename_byte_len, abs_path_len): (usize, usize),
    (filename_len_valid, abs_file_path_len_valid): (bool, bool),
) {
    println!(
        "Filename len: {}/{} {}",
        filename_byte_len, MAX_FILE_LEN, RESULT_EMOJII[filename_len_valid as usize],
    );
    println!(
        "Absolute path len: {}/{} {}",
        filename_byte_len + abs_path_len,
        MAX_ABS_PATH_LEN,
        RESULT_EMOJII[abs_file_path_len_valid as usize],
    );
}

pub fn print_doc() {
    println!("Check if it possible to:");
    println!(
        "1. create <filename> in <dir>: cfl <filename> <dir>
(You can avoid passing dir argument if you suggest the current one)"
    );
    println!("2. copy/move <filename> in <dest_path>: cfl -l <filename> <dest_path>");
}
