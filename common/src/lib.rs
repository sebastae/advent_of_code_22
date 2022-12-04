use std::{env, fs, process::exit};

pub fn read_input_file_from_args(arg_num: usize) -> Result<String, String> {

    let args: Vec<String> = env::args().collect();
    if args.len() < arg_num {
        return Err(format!("Too few arguments, expected {arg_num}"));
    }

    let file_path = args.get(arg_num).expect("Too few arguments");
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(content) => Ok(content),
        Err(err) => {
            Err(err.to_string())
        }
    }
}

pub fn get_file_content_or_exit() -> String{
    let input = read_input_file_from_args(1);
    if let Err(err) = input {
        println!("{}", err);
        exit(1);
    }

    input.unwrap()
}
