pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub fn subtractio(first: u64, second: u64)-> u64{
    first - second
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_addtion_test() {
        let result = add(5, 4);
        assert_eq!(result, 9);
        print!("passed first test :✅");

    }

    #[test]
    fn test_subtraction(){
        let result = subtractio(5,2);
        assert_eq!(result,3);
        print!("passed second test :✅");
    }

}
