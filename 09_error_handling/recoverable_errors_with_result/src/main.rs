use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    //let greeting_file_result = File::open("hello.txt");
    //
    //let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //    Err(error) => panic!("Problem opening the file: {:?}", error),
    //};

    // Matching on Different Errors
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // Shortcuts for Panic on Error: unwrap and expect
    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file =
        File::open("hello2.txt").expect("hello.txt should be included in this project");

    // Where The ? Operator Can Be Used
    // compile error cannot use the `?` operator in a function that returns `()`
    // let greeting_file = File::open("hello.txt")?;
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// A shortcut for Propagating Errlrs: the ? Operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
