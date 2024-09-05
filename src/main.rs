use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    reverse_file_content()
}

fn reverse_file_content() -> io::Result<()> {
    println!("Provide a file path: ");

    let mut file_path = String::new();

    io::stdin().read_line(&mut file_path)?;

    let file_path_str = file_path.as_str().trim();

    if !file_path_str.ends_with(".txt") {
        println!("Please provide a valid .txt file path");
        return reverse_file_content();
    }

    let mut file = File::open(file_path_str)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    let file_path = Path::new(file_path_str);
    let mut output_file_base: &str = "";
    let output_file_name_format: String;
    let output_file_name: &str;

    if let Some(file_name) = file_path.file_name() {
        output_file_base = file_name.to_str().unwrap();
    }

    if output_file_base.len() > 0 {
        output_file_name_format = format!("output-{}", output_file_base);
        output_file_name = output_file_name_format.as_str();
    } else {
        output_file_name = "output.txt"
    }

    let mut output_file: File = File::create(output_file_name)?;

    let mut output_file_content = String::new();
    let mut output_file_content_index = file_content.len();

    while output_file_content_index > 0 {
        output_file_content_index -= 1;

        if let Some(c) = file_content.chars().nth(output_file_content_index) {
            output_file_content.push(c);
        }
    }

    output_file.write_all(output_file_content.as_bytes())?;

    Ok(())
}
