use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = cfl::collect_user_args().skip(1);
    let program_result = match args.len() {
        1 => cfl::is_file_len_valid(&args.next().unwrap(), cfl::get_current_working_dir()?)?,
        2 => cfl::is_file_len_valid(
            &args.next().unwrap(),
            cfl::get_dir_from_str(&args.next().unwrap())?,
        )?,
        3 => todo!(),
        _ => {
            cfl::print_doc();
            true
        }
    };
    cfl::exit_program(program_result)
}
