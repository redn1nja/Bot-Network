use std::env::consts::OS;

use libloading::{Library, Symbol};

fn main() {
    let mut LIB_EXTENSION = "";
    match OS {
        "macos" => {
            LIB_EXTENSION = "dylib";
        }
        "windows" => {
            LIB_EXTENSION = "dll";
        }
        "linux" => {
            LIB_EXTENSION = "so";
        }
        _ => {
            panic!("Unsupported OS");
        }
    }

    let mut library_path = format!("../src/client/target/debug/libclient_lib.{}", LIB_EXTENSION);

    loop {
        unsafe {
            let lib = Library::new(&library_path).expect("Failed to load dynamic library");
            let main_fun: Symbol<extern "C" fn() -> bool> =
                lib.get(b"startup").expect("Failed to load function");
            let result = main_fun();

            let new_library_path = format!("/tmp/libclient_lib.{}", LIB_EXTENSION);
            if std::path::Path::new(&new_library_path).exists() {
                lib.get::<Symbol<extern "C" fn()>>(b"shutdown")
                    .expect("Failed to get shutdown function")();
                drop(lib);

                library_path = new_library_path.to_owned();

                std::fs::remove_file(&new_library_path).expect("Failed to remove new library file");
            }
        }
    }
}
