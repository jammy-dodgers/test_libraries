use libloading::{Library, Symbol};
use std::env::args;

fn main() {
    let lib_filename = args().nth(1).expect("Requires path argument");
    println!("Loading {}", lib_filename);
    let lib = Library::new(lib_filename).expect("Could not load library");
    
    unsafe {
        let func: Symbol<extern "C" fn(u64) -> ()> = lib.get(b"export")
            .expect("Could not find 'export' symbol");
        func(43);
    }
}
