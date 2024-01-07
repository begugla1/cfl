use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = cfl::collect_user_args().skip(1);
    let mut exit_code = 0;
    match args.len() {
        1 => {
            exit_code = cfl::is_file_len_valid(
                &args.next().unwrap(), cfl::get_current_working_dir()?
            )? as i32;
        },
        2 => {
            exit_code = cfl::is_file_len_valid(
                &args.next().unwrap(), cfl::get_dir_from_str(&args.next().unwrap())?
            )? as i32;
        }
        3 => todo!(),
        _ => cfl::print_doc(),
    }
    cfl::exit_program(exit_code)
}
