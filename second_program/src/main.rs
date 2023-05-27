use libloading::{Library, Symbol};


fn main() {
    unsafe {
        let lib = Library::new("../src/host/target/release/libhost_lib.so").expect("Failed to load dynamic library");

        // Load the function from the library
        let main_fun: Symbol<extern "C" fn()> = lib.get(b"main").expect("Failed to load function");

        // Call the function
        println!("Result");
        main_fun();
    }
}
