enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn read_username_from_file() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        // Err(e) => Err(e),
    }

    println!("{}", s);

}

fn main() {
    // panic!("crash and burn");
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };
    read_username_from_file();
}
