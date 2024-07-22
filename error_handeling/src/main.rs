use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Prolem opening the file: {:?}", other_error)
            }
        }
    };

    let x = 42;
    if x ==42 {
        panic!("The Hitchhiker's guide is wrong!")
    };

}
