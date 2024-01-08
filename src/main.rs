use std::io;

fn main() -> io::Result<()> {
    let args = cfl::collect_user_args();
    let mut exit_code = cfl::ExitCode(0);
    if args.len() == 1 {
        cfl::print_doc();
    } else {
        let path = cfl::build_path(args)?;
        let analysis_result = cfl::make_file_len_analysis(path);
        cfl::print_file_len_analysis_result(&analysis_result);
        exit_code = cfl::generate_exit_code(&analysis_result);
    }
    cfl::exit_program(exit_code)
}
