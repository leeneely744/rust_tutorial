use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // let file_open_result = File::open("hello.txt");  // non exist file.
    // file_open_result is Result<T,E>
    // If File::open succeeds, file_open_result is an instance of Ok.
    // If it fails, an instance of Err.

    // let _file = match file_open_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // this need `use std::io::ErrorKind;`
    // let _file2 = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // most Rustaceans choose 'expect' rather than 'unwrap'.
    let _file3 = File::open("hello.txt").expect("my error message");
}
