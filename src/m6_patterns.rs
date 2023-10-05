


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_match_literals() {
        let number: i32 = 20;

        let res: &str = match number {
            1 => "It was the first!",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!",
            _ => "It was something else!",
        };

        println!("{}",res);
    }

    #[test]
    fn test_match_option() {
        
    }
}