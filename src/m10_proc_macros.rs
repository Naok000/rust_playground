#[cfg(test)]
mod tests {
    use ::my_proc_macro::function_to_string;
    use ai_functions::ai_function;

    const OUTPUT: &str = "";


    #[ai_function]
    fn another_ai_function(_whatever_param: &str) {
        // This is an awesome function, from the crates.io libraries
        // We shall give it to an AI to guess and output
        // In a structured manner
        println!("{}", OUTPUT);
    }

    #[function_to_string]
    fn some_function_for_ai_lim(_whatever_param: &str) {
        // This is an awesome function
        // We shall give it to an AI to guess and output
        // In a structured manner
        println!("{}", OUTPUT);
    }

    #[test]
    fn tests_proc_macro() {
        let x: &str = some_function_for_ai_lim("some_input");
        let y: &str = another_ai_function("some_input_again");
        dbg!(x);
        dbg!(y);
    }
}
