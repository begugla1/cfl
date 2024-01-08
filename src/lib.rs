mod analysis;
mod output;
mod path;
mod utils;

use std::{os::unix::ffi::OsStrExt, path::PathBuf};

use analysis::AnalysisResult;

pub use output::{print_doc, print_file_len_analysis_result};
pub use path::build_path;
pub use utils::{collect_user_args, exit_program, generate_exit_code, ExitCode};

const MAX_FILE_LEN: usize = 255;
const MAX_ABS_PATH_LEN: usize = 4096;

/// Checking if existing of file with given `abs_path` is possible
pub fn make_file_len_analysis(abs_path: PathBuf) -> AnalysisResult {
    let filename = abs_path
        .components()
        .last()
        .expect("File has no any components");
    let filename_len = filename.as_os_str().as_bytes().len();
    let abs_path_len = abs_path.as_os_str().as_bytes().len();
    let filename_len_valid = filename_len <= MAX_FILE_LEN;
    let abs_path_len_valid = abs_path_len <= MAX_ABS_PATH_LEN;
    AnalysisResult::new(
        filename_len,
        abs_path_len,
        filename_len_valid,
        abs_path_len_valid,
        abs_path,
    )
}
