use std::io::{self, BufRead, Write};
use serde_json::{self, json, Value};
use uuid::Uuid;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let uuid = Uuid::new_v4();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    let mut msg_id = 1;

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
                let msg_type: String = object["body"]["type"].as_str().unwrap().to_string();
                match msg_type.as_str() {
                    "init" => object["body"].as_object_mut().unwrap().insert("type".to_string(), json!("init_ok")),
                    "generate" => {
                        object["body"].as_object_mut().unwrap().insert("type".to_string(), json!("generate_ok"));
                        let uuid = Uuid::new_v4();
                        object["body"]["id"] = json!(uuid.to_string());
                        None
                    }
                    "echo" => object["body"].as_object_mut().unwrap().insert("type".to_string(), json!("echo_ok")),
                     _ => {
                         println!("No match! Type: {}", msg_type.as_str());
                         None
                     }
                };
                object["body"]["in_reply_to"] = json!(object["body"]["msg_id"]);
                object["body"]["msg_id"] = json!(msg_id);

                msg_id += 1;

                eprintln!("{}", serde_json::to_string(&object).unwrap());
                writeln!(std::io::stdout(), "{}", serde_json::to_string(&object).unwrap()).unwrap();
                std::io::stdout().flush().unwrap();
            }
            Err(error) => {
                eprintln!("Error reading from stdin: {}", error);
                break;
            }
        }
    }
}
