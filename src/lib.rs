#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}
