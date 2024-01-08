use std::{io, os::unix::ffi::OsStrExt, path::PathBuf};

pub fn build_path(mut args: std::env::Args) -> io::Result<PathBuf> {
    args.next();
    let raw_filename = get_path_from_str(&args.next().expect("Filename argument expected"), false)?;
    let mut filename = raw_filename.clone();
    if filename.exists() {
        filename = find_longest_path(filename)?;
    }
    let mut dest_dir = get_current_working_dir()?;
    if let Some(dirname) = args.next() {
        dest_dir = get_path_from_str(&dirname, true)?;
    }
    let parsed_filename = parse_path(filename, raw_filename);
    dest_dir.push(parsed_filename);
    Ok(dest_dir)
}

fn find_longest_path(dir: PathBuf) -> io::Result<PathBuf> {
    let mut longest_abs_path = dir.clone();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                longest_abs_path = get_max_abs_path(longest_abs_path, find_longest_path(path)?);
            } else {
                longest_abs_path = get_max_abs_path(longest_abs_path, path);
            }
        }
    }
    Ok(longest_abs_path)
}

fn get_max_abs_path(path1: PathBuf, path2: PathBuf) -> PathBuf {
    let path1_len = path1.as_os_str().as_bytes().len();
    let path2_len = path2.as_os_str().as_bytes().len();
    if path1_len > path2_len {
        path1
    } else {
        path2
    }
}

fn get_path_from_str(dirname: &str, is_abs: bool) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(dirname);
    if is_abs {
        path = path.canonicalize()?;
    }
    Ok(path)
}

fn parse_path(path: PathBuf, prefix: PathBuf) -> PathBuf {
    let ctr = prefix.components().count();
    path.components().skip(ctr - 1).collect()
}

fn get_current_working_dir() -> io::Result<PathBuf> {
    get_path_from_str(".", true)
}
