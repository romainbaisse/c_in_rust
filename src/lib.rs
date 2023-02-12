pub fn add(left: usize, right: usize) -> usize {
    left + right
}
extern {
    fn say_hello();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn hello_test() {
        hello();
    }
}

pub fn hello() {
    println!("Hello from rust code");
    unsafe {
        say_hello();
    }
}