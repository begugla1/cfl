mod output;
mod utils;

use std::error::Error;

pub use output::print_doc;
pub use utils::{collect_user_args, exit_program};

const MAX_FILE_LEN: usize = 255;
const MAX_ABS_PATH_LEN: usize = 4096;
const RESULT_EMOJII: [char; 2] = ['❌', '✅'];

/// Checking if it possible to create file with given name in current directory
/// based on it's length and length of absolute path with this file.
pub fn is_file_len_valid_in_curr_dir(filename: &str) -> Result<bool, Box<dyn Error>> {
    if !utils::filename_is_unique(filename, &utils::get_current_working_dir()?)? {
        return Err(Box::from(format!(
            "File with name `{}` already exists",
            filename
        )));
    }
    let filename_byte_len = filename.bytes().len();
    let abs_path_len = utils::get_current_working_dir()?
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
    Ok(abs_file_path_len_valid && filename_len_valid)
}
