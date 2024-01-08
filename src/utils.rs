use crate::AnalysisResult;
pub struct ExitCode(pub i32);

pub fn generate_exit_code(analisys_result: &AnalysisResult) -> ExitCode {
    ExitCode(!analisys_result.is_successful() as i32)
}

pub fn collect_user_args() -> std::env::Args {
    std::env::args()
}

pub fn exit_program(exit_code: crate::ExitCode) -> ! {
    std::process::exit(exit_code.0)
}
