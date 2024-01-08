use std::path::PathBuf;

pub struct AnalysisResult {
    pub filename_len: usize,
    pub abs_path_len: usize,
    pub filename_len_valid: bool,
    pub abs_path_len_valid: bool,
    pub abs_path: PathBuf,
}

impl AnalysisResult {
    pub fn new(
        filename_len: usize,
        abs_path_len: usize,
        filename_len_valid: bool,
        abs_path_len_valid: bool,
        abs_path: PathBuf,
    ) -> Self {
        Self {
            filename_len_valid,
            abs_path_len_valid,
            filename_len,
            abs_path_len,
            abs_path,
        }
    }
    pub fn is_successful(&self) -> bool {
        self.filename_len_valid && self.abs_path_len_valid
    }
}
