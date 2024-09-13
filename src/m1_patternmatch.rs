


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_enum() {
        let some_num = Some(10);

        let res = match some_num {
            Some(num) => num,
            None => panic!("Error")
        };

        dbg!("res: {:?}", res);
    }
}

