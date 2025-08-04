fn main() {
    println!("run tests instead")
}

fn fizz_buzz(number: i32) -> String {
    if number % 3 == 0 {
        return "Fizz".to_string()
    } else if number % 5 == 0 {
        return "Buzz".to_string()
    }

    number.to_string()
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