mod output;
mod utils;

use std::{error::Error, os::unix::ffi::OsStrExt, path::PathBuf};

pub use output::print_doc;
pub use utils::{build_path, collect_user_args};

const MAX_FILE_LEN: usize = 255;
const MAX_ABS_PATH_LEN: usize = 4096;

/// Checking if existing of file with given `filename` is possible
pub fn is_file_len_valid(abs_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let filename = abs_path
        .components()
        .last()
        .expect("File has no any components");
    let filename_len = filename.as_os_str().as_bytes().len();
    let abs_path_len = abs_path.as_os_str().as_bytes().len();
    let filename_len_valid = filename_len <= MAX_FILE_LEN;
    let abs_path_len_valid = abs_path_len <= MAX_ABS_PATH_LEN;
    output::print_file_len_analisys_result(
        (filename_len, abs_path_len),
        (filename_len_valid, abs_path_len_valid),
        abs_path,
    );
    utils::exit_program(!(filename_len_valid && abs_path_len_valid) as i32)
}
