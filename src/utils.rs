use std::{env, fs, path::PathBuf, error::Error, io};

pub fn filename_is_unique(filename: &str, path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if filename == entry.file_name() {
            return Ok(false)
        }
    }
    Ok(true)
}

pub fn get_current_working_dir() -> io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_dir_from_str(dirname: &str) -> Result<PathBuf, io::Error> {
    PathBuf::from(dirname).canonicalize()
}

pub fn collect_user_args() -> env::Args {
    env::args()
}

pub fn exit_program(exit_code: i32) -> ! {
    std::process::exit(exit_code)
}
