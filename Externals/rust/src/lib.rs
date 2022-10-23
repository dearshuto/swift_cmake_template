pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn print_rs() {
    println!("Hello World from Rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
