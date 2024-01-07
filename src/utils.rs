use std::{env, error::Error, fs, io, path::PathBuf};

pub fn filename_is_unique(filename: &str, path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if filename == entry.file_name() {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn get_current_working_dir() -> io::Result<PathBuf> {
    PathBuf::from(".").canonicalize()
}

pub fn get_dir_from_str(dirname: &str) -> Result<PathBuf, io::Error> {
    PathBuf::from(dirname).canonicalize()
}

pub fn collect_user_args() -> env::Args {
    env::args()
}

/// If `program_result` is true, then program will exit with 0 code, otherwise with 1 code
pub fn exit_program(program_result: bool) -> ! {
    std::process::exit(!program_result as i32)
}
