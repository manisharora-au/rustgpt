
enum Message {
    Quit,
    ChangeColour(i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) -> String {
    match msg {
        Message::Quit => format!("Quit"),
        Message::ChangeColour(r, g) => format!("Change colour from {} to {}", r, g),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(text) => format!("Write: {}", text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_enum_message() {
        let msg_change_color = Message::ChangeColour(255, 0);
        assert_eq!(process_message(msg_change_color), String::from("Change colour from 255 to 0"));

        let msg_move = Message::Move { x: 10, y: 20 };
        assert_eq!(process_message(msg_move), "Move to (10, 20)");

        let msg_write = Message::Write("Hello, World!".to_string());
        assert_eq!(process_message(msg_write), "Write: Hello, World!");

        let msg_quit = Message::Quit;
        assert_eq!(process_message(msg_quit), "Quit");
    }

    #[test]
    fn test_option_enum() {
        let some_num = Some(250);
        // let some_num: Option<i32> = None;

        let my_result = match some_num {
            Some(num) => num,
            None => panic!("Error")
        };

        dbg!("my_result is {}", my_result);
    }

    #[test]
    fn test_result_enum() {
        let some_num: Result<i32, String> = Ok(20);
        // let some_err: Result<i32, &str> = Err("There was an error");

        let my_result = match some_num {
            Ok(i) => i,
            Err(e) => panic!("{}", e)
        };

        dbg!("my_result is {}", my_result);
    }

    #[test]
    fn test_match_guard() {
        let pair = (2, -2);
        match pair {
            (x, y) if x > 0 && y > 0 => println!("Both numbers are positive"),
            (x, y) if x < 0 && y < 0 => println!("Both numbers are negative"),
            (x, y) if x == 0 || y == 0 => println!("One number is zero"),
            (x, y) if x == 2 && y == -2 => println!("Two numbers match"),
            _ => println!("Neither number is zero nor positive or negative"),
        }
    }

    #[test]
    fn test_match_struct() {
        #[derive(PartialEq, Debug)]
        enum Location_Type {
            Point,
            Line,
            Polygon,
        }
        #[derive(PartialEq, Debug)]
        struct Location {
            x: i32,
            y: i32,
            loc_type: Location_Type
        }

        let name = String::new();
        let loc1 = Location { x: 1, y: 2, loc_type: Location_Type::Line};
        let loc2 = Location { x: 1, y: 2, loc_type: Location_Type::Polygon};
        assert_eq!(loc1, loc2);
    }



}

