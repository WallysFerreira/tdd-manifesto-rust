fn main() {
    println!("run tests instead")
}

fn fizz_buzz(number: i32) -> String {
    if number_is_multiple_of(number, 3) {
        return "Fizz".to_string()
    } else if number_is_multiple_of(number, 5) {
        return "Buzz".to_string()
    }

    number.to_string()
}

fn number_is_multiple_of(number: i32, multiplier: i32) -> bool {
    number % multiplier == 0
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
}