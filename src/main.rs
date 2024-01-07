use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cfl::collect_user_args();
    if args.len() == 1 {
        cfl::print_doc();
    } else {
        let path = cfl::build_path(args)?;
        cfl::is_file_len_valid(path)?;
    }
    Ok(())
}
