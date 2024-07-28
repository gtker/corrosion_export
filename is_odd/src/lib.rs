#[no_mangle]
pub extern "C" fn is_odd(number: i32) -> bool {
    number % 2 == 1
}