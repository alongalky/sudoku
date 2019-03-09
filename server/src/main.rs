#![feature(proc_macro_hygiene, decl_macro)]
extern crate rand;
extern crate serde_derive;
 
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde_json;
use serde_json::json;

mod boards;
use boards::BOARDS;

#[derive(Clone)]
struct Game {
    name: String,
    state: String
}

impl Game {
    fn update_digit(&mut self, icol: u8, irow: u8, digit: u8) {
        let index: usize = (irow * 9 + icol) as usize;
        self.state = self.state[..index].to_string() + &digit.to_string() + &self.state[(index+1)..].to_string();
    }
}

type ID = String;
type MessageMap = Arc<Mutex<HashMap<ID, Game>>>;

use ws::listen;
use ws::{Factory, Error, Handler, Handshake, CloseCode, Sender, Result, Message};
use env_logger;
const GAME_ID: &str = "abcde";

struct Server {
    out: Sender,
    games: MessageMap
}

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct SendDigitMessage {
    icol: u8,
    irow: u8,
    digit: u8
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let mut hashmap = self.games.lock().unwrap();
        if !hashmap.contains_key(GAME_ID) {
            println!("found key! {}", GAME_ID);
            let mut rng = thread_rng();
            let game = Game {
                name: GAME_ID.to_string(),
                state: BOARDS.choose(&mut rng).unwrap().to_string()
            };
            hashmap.insert(GAME_ID.to_string(), game.clone());
        }
        let game = hashmap.get(GAME_ID).unwrap();
        println!("new connection");
        
        self.out.send(Message::text(json!({
            "state": &game.state,
            "gameName": &game.name
        }).to_string()));
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client says {}", msg);

        if let Message::Text(txt) = msg.clone() {
            let v: serde_json::Value = serde_json::from_str(&txt).unwrap();
            
            if let typ = &v["type"] {
                if typ == "send_digit" {
                    let data: SendDigitMessage = serde_json::from_str(&txt).unwrap();

                    let mut hashmap = self.games.lock().unwrap();
                    let mut game = hashmap.get_mut(GAME_ID).unwrap();
                    game.update_digit(data.icol, data.irow, data.digit);

                    self.out.send(Message::text(json!({
                        "state": &game.state,
                        "gameName": &game.name
                    }).to_string()));
                }
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        // self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

}

fn main() {
    env_logger::init();
    let mut map: MessageMap = Arc::new(Mutex::new(HashMap::<ID, Game>::new()));

    listen("127.0.0.1:3012", |out| { Server { out: out, games: map.clone() } }).unwrap()
}