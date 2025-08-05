fn main() {
    println!("run tests instead")
}

fn fizz_buzz(number: i32) -> String {
    match (is_multiple_of(number, 3), is_multiple_of(number, 5)) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => number.to_string(),
    }
}

fn is_multiple_of(number: i32, divisor: i32) -> bool {
    number % divisor == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_number_as_string() {
        let number = 322;
        let expected = number.to_string();
        let actual = fizz_buzz(number);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_should_return_fizz_for_multiples_of_three() {
        let number = 27;
    
        assert_eq!(fizz_buzz(number), "Fizz")
    }

    #[test]
    fn test_should_return_buzz_for_multiples_of_five() {
        let number = 10;

        assert_eq!(fizz_buzz(number), "Buzz")
    }

    #[test]
    fn test_should_return_fizz_buzz_for_multiples_of_five_and_three() {
        let number = 30;

        assert_eq!(fizz_buzz(number), "FizzBuzz")
    }
}