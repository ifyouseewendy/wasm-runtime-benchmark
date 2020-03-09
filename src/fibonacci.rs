#[no_mangle]
pub extern "C" fn ext_run(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        run(n - 1) + run(n - 2)
    }
}

pub fn run(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        run(n - 1) + run(n - 2)
    }
}
