use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

fn main() -> Result<()> {
    read_all_content_from_a_file();

    let temp_list = read_content_line_by_line_from_a_file().unwrap();
    println!("temp_list: {:?}", temp_list);

    execute_command();

    Ok(())
}

///
fn read_all_content_from_a_file() -> Result<String> {
    //
    // `env::current_dir` return a `PathBuf` instance, you can
    // use `push` to add extra path to it. But:
    //
    // If you push a absolute path, it will **replace** the exists one!!!
    //
    let mut data_file_name = env::current_dir()?;
    data_file_name.push("src/test.dat");
    println!("data_file_name: {:?}", data_file_name);

    //
    // Open a file and read all content into a big string
    // After we opened the file, we don't need to close it,
    // the `FD` will be closed automatic when it outs of
    // the current scope!!!
    //
    let data_file = File::open(data_file_name)?;
    let mut file_reader = BufReader::new(data_file);
    let mut all_content = String::new();
    file_reader.read_to_string(&mut all_content)?;
    Ok(all_content)
}

///
fn read_content_line_by_line_from_a_file() -> Result<Vec<usize>> {
    let mut data_file_name = env::current_dir()?;
    data_file_name.push("src/test.dat");

    let data_file = File::open(data_file_name)?;
    let file_reader = BufReader::new(data_file);

    let mut id_list: Vec<usize> = Vec::new();
    for line in file_reader.lines() {
        let temp_line = line?;

        // content is `xxxx,yyyy`
        let temp_arr = temp_line.split(",").collect::<Vec<&str>>();
        if temp_arr.len() > 1 {
            // Convert the second string to `u32`
            // println!("temp value: {}", temp_arr[1]);
            id_list.push(temp_arr[1].parse::<usize>().unwrap());
        }
    }

    Ok(id_list)
}

///
fn execute_command() {
    use std::process::Command;

    let text_to_speak = "Hey, I can speak:)";
    let mut speak_command = Command::new("say");
    let execute_reuslt = speak_command.arg(text_to_speak).output();
    println!("execute_reuslt: {:?}", execute_reuslt);
}
