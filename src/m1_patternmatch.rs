enum Message {
    Quit,
    ChangeColour(i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColour(r, g) => println!("Change colour from {} to {}", r, g),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_message() {
        let msg_change_color = Message::ChangeColour(255, 0);
        process_message(msg_change_color);

        let msg_move = Message::Move { x: 10, y: 20 };
        process_message(msg_move);

        let msg_write = Message::Write("Hello, World!".to_string());
        process_message(msg_write);

        let msg_quit = Message::Quit;
        process_message(msg_quit);
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



}

