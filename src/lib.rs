use serde_json::json;
use slack_flows::{listen_to_channel, send_message_to_channel};
use store_flows::{del, get, set};

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "chatgpt", |sm| {
        let last_result = match sm.text == "C" {
            true => {
                del("last_result");
                return;
            }
            false => match get("last_result") {
                Some(v) => v.as_f64().unwrap_or_default(),
                None => 0.0,
            },
        };
        let expr = match sm.text.chars().next() {
            Some(c) => match c {
                '+' | '-' | '*' | '/' => {
                    format!("{}{}", last_result, sm.text)
                }
                _ => sm.text,
            },
            None => sm.text,
        };
        match meval::eval_str(expr) {
            Ok(v) => {
                set("last_result", json!(v));
                send_message_to_channel("secondstate", "chatgpt", v.to_string());
            }
            Err(_) => {
                send_message_to_channel("secondstate", "chatgpt", String::from("Invalid expression"));
            }
        }
    });
}
