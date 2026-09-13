use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = File::open("test.txt").expect("test.txt should always be part of the program.d");
    /*let result = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("The file was not found");
            _ => panic!("Creation of the file lead to a generic error");
        }
    }*/
}
