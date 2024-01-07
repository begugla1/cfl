mod output;
mod utils;

use std::{error::Error, path::PathBuf};

pub use output::print_doc;
pub use utils::{collect_user_args, build_path};

const MAX_FILE_LEN: usize = 255;
const MAX_ABS_PATH_LEN: usize = 4096;
const RESULT_EMOJII: [char; 2] = ['❌', '✅'];

/// Checking existing of given `filename`
pub fn is_file_len_valid(filename: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!()
}
