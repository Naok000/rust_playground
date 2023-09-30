#[derive(Debug)]
enum CarColur {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T,E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn create_car_colour_blue() -> CarColur {
    let my_car_colour: CarColur = CarColur::Blue;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8,String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn check_under_five_built_in(num_check: u8) -> Result<u8,String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}

fn remainder_zero(num_check:f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_built_in(num_check:f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Option::Some(remainder)
    } else {
        Option::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour: CarColur = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_res:Result<u8, String> = check_under_five_built_in(2);
        dbg!(under_five_res);

        let under_five_res:Result<u8, String> = check_under_five_built_in(7);
        dbg!(under_five_res);

        let remainder: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(remainder);
    }
}
