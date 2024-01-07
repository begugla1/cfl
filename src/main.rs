use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = cfl::collect_user_args().skip(1);
    match args.len() {
        0 => cfl::print_doc(),
        1 => {
            let filename_is_valid =
                cfl::is_file_len_valid(&args.next().expect("Filename argument expected"))?;
            if !filename_is_valid {
                cfl::exit_program(1)
            }
        }
        _ => todo!(),
    }
    Ok(())
}
