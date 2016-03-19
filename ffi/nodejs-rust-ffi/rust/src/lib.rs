#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
