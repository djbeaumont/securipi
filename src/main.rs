use std::env;
use twilio::{Client, OutboundMessage};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let account_id = env::var("ACCOUNT_ID").unwrap();
    let auth_token = env::var("AUTH_TOKEN").unwrap();
    let sms_from = env::var("SMS_FROM").unwrap();
    let sms_to = env::var("SMS_TO").unwrap();

    let client = Client::new(&account_id, &auth_token);
    let message = "Hello, World!";
    match client.send_message(OutboundMessage::new(&sms_from, &sms_to, message)) {
        Ok(_) => println!("Sent!"),
        Err(_) => println!("Not sent!"),
    }
}
