mod output;
mod utils;

use std::{error::Error, path::PathBuf};

pub use output::print_doc;
pub use utils::{collect_user_args, exit_program, get_current_working_dir, get_abs_dir_from_str};

const MAX_FILE_LEN: usize = 255;
const MAX_ABS_PATH_LEN: usize = 4096;
const RESULT_EMOJII: [char; 2] = ['❌', '✅'];

/// Checking if it possible to create file with  `filename` in `dir`
/// based on it's length and length of absolute path with this file.
pub fn is_file_len_valid(filename: &str, mut dir: PathBuf) -> Result<bool, Box<dyn Error>> {
    if !utils::filename_is_unique(filename, &dir)? {
        return Err(Box::from(format!(
            "File with name `{}` already exists",
            filename
        )));
    }
    let filename_byte_len = filename.bytes().len();
    let abs_path_len = dir
        .as_mut_os_str()
        .to_str()
        .expect("Can't decode absolute path to Unicode")
        .len();
    let filename_len_valid = filename_byte_len <= MAX_FILE_LEN;
    let abs_file_path_len_valid = filename_byte_len + abs_path_len <= MAX_ABS_PATH_LEN;
    output::print_file_len_analisys_result(
        (filename_byte_len, abs_path_len),
        (filename_len_valid, abs_file_path_len_valid),
    );
    Ok(filename_len_valid && abs_file_path_len_valid)
}
