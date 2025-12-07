use std::{
    env,
    fs,
    fs::File,
    path::Path
};

const INVALID_ARGS_MESSAGE: &str = "Please pass in year and day as args.";

const NEW_DAY_FILE_TEMPLATE: &str = "boilerplate.rs";
const INPUT_FILE_NAME: &str = "input.txt";
const PROGRAM_FILE_NAME: &str = "main.rs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("{}", INVALID_ARGS_MESSAGE);
    }
    let year = &args[1];
    let day = &args[2];
    let dir_path = Path::new(year).join(format!("day_{day}"));
    let input_file_path = dir_path.join(INPUT_FILE_NAME);
    let program_file_path = dir_path.join(PROGRAM_FILE_NAME);
    if input_file_path.exists() {
        println!("File {} already exists", input_file_path.display());
        return Ok(());
    }
    if program_file_path.exists() {
        println!("File {} already exists", program_file_path.display());
        return Ok(());
    }
    fs::create_dir_all(dir_path)?;
    let mut input_file = File::create(&input_file_path)?;
    // let mut program_file = File::create(&program_file_path)?;
    let boilerplate_code_file = Path::new(NEW_DAY_FILE_TEMPLATE);
    fs::copy(&boilerplate_code_file, &program_file_path)?;
    Ok(())

}
