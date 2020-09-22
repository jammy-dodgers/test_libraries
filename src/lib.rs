#[no_mangle]
pub extern "C" fn export(v: i32) -> () {
    println!("External call! {}", v);
}
