use std::io::{self, BufRead};
use serde_json::{self, json, Value};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();

        match handle.read_line(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break; // end of input
                }
                let mut object: Value = serde_json::from_str(&buffer).unwrap();

                // Swap the values of "src" and "dest" keys
                let src = object["src"].take();
                object["src"] = object["dest"].take();
                object["dest"] = src;

                // Modify the "body" object to include the desired fields
                let body = object["body"].as_object_mut().unwrap();
                body.insert("type".to_string(), json!("echo_ok"));
                body.insert("in_reply_to".to_string(), body["msg_id"].clone());

                println!("{}", serde_json::to_string_pretty(&object).unwrap());
            }
            Err(error) => {
                eprintln!("Error reading from stdin: {}", error);
                break;
            }
        }
    }
}
