use super::{AnalysisResult, MAX_ABS_PATH_LEN, MAX_FILE_LEN};

pub fn print_file_len_analysis_result(
    AnalysisResult {
        filename_len,
        abs_path_len,
        filename_len_valid,
        abs_path_len_valid,
        abs_path,
    }: &AnalysisResult,
) {
    println!("Considered file: '{}'", abs_path.display());
    let filename_result_template = if *filename_len_valid {
        "Filename len valid ✅"
    } else {
        "Filename len invalid ❌"
    };
    let abs_path_result_template = if *abs_path_len_valid {
        "Absolute file path len valid ✅"
    } else {
        "Absolute file path len invalid ❌"
    };
    println!(
        "{} {}/{} bytes",
        filename_result_template, filename_len, MAX_FILE_LEN
    );
    println!(
        "{} {}/{} bytes",
        abs_path_result_template, abs_path_len, MAX_ABS_PATH_LEN
    );
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
