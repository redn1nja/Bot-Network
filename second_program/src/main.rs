use libloading::{Library, Symbol};
fn main() {
    unsafe {
        loop {
            let lib = Library::new("../src/client/target/debug/libclient_lib.so").expect("Failed to load dynamic library");
            let main_fun: Symbol<extern "C" fn()->bool> = lib.get(b"startup").expect("Failed to load function");
            let result = main_fun();
        }

    }
}
