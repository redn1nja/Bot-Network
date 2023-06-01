use std::env::consts::OS;
use std::time::Duration;

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
            println!("loadedold");
            let lib = Library::new(&library_path).expect("Failed to load dynamic library");
            let main_fun: Symbol<extern "C" fn()> =
                lib.get(b"startup").expect("Failed to load function");
            main_fun();
            let new_library_path = format!("/tmp/libclient.{}", LIB_EXTENSION);
            std::thread::sleep(Duration::from_secs(10));
            if std::path::Path::new(&new_library_path).exists() {
                let r = lib.close();
                println!("updating");

                library_path = new_library_path.to_owned();
                std::fs::remove_file(&new_library_path).expect("Failed to remove new library file");
            }
        }
    }
}
