#[derive(Debug)]
#[derive(PartialEq)]
enum CarColor {
    Red,
    Blue,
    Green,
}

fn create_car_color_blue() -> CarColor {
    CarColor::Blue
}
fn create_car_color_green() -> CarColor {
    CarColor::Green
}
fn create_car_color_red() -> CarColor {
    CarColor::Red
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GetResult<T, E> {
    Ok(T),
    Err(E),
}


fn check_result_if_less_than_five(num: u8) -> GetResult<u8, String> {
    match num {
        0..=4 => GetResult::Ok(num),
        Other => GetResult::Err(format!("more than 5")),
    }
}

fn check_result_if_less_than_five_builtin(num: u8) -> Result<u8, String> {
    match num {
        0..=4 => Ok(num),
        Other => Err(format!("more than 5")),
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GetOutput<T> {
    Some(T),
    None,
}

fn check_output(num: i16) -> GetOutput<i16> {
    match num {
        0..=i16::MAX => GetOutput::Some(num), // For all values >= 0
        Other => GetOutput::None,       // Wildcard for other cases
    }
}

fn check_output_builtin(num: i16) -> Option<i16> {
    match num {
        0..=i16::MAX => Some(num), // For all values >= 0
        Other => None,       // Wildcard for other cases
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_blue() {
        let color_blue = create_car_color_blue();
        assert_eq!(color_blue, CarColor::Blue);
    }

    #[test]
    fn test_num_less_than_five() {
        let result = check_result_if_less_than_five(4);
        assert_eq!(result, GetResult::Ok(4));

        let result = check_result_if_less_than_five(5);
        assert_eq!(result, GetResult::Err(format!("more than 5")));

        let result = check_result_if_less_than_five_builtin(4);
        assert_eq!(result, Ok(4));

        let result = check_result_if_less_than_five_builtin(5);
        assert_eq!(result, Err(format!("more than 5")));
    }

    #[test]
    fn test_output_some() {
        let result = check_output(1000);
        assert_eq!(result, GetOutput::Some(1000));

        let result = check_output(-20);
        assert_eq!(result, GetOutput::None);

        let result = check_output_builtin(1000);
        assert_eq!(result, Some(1000));

        let result = check_output_builtin(-50);
        assert_eq!(result, None);
    }
}