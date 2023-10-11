


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
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);
        let prob_none: Option<i32> = None;

        let res = match some_num {
            Some(i) => i,
            None => {
                panic!("There was a problem");
            }
        };

        println!("{:?}",some_num);
        println!("{}",res);
    }
}