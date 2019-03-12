#![feature(proc_macro_hygiene, decl_macro)]
extern crate rand;
extern crate serde_derive;
extern crate uuid;
use uuid::Uuid;
 
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
    state: String,
    players: Vec<Player>
}

impl Game {
    fn update_digit(&mut self, icol: u8, irow: u8, digit: u8) {
        let index: usize = (irow * 9 + icol) as usize;
        self.state = self.state[..index].to_string() + &digit.to_string() + &self.state[(index+1)..].to_string();
    }

    fn add_player(&mut self, player: Player) {
        println!("Adding {}", &player.name);
        self.players.push(player);
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Player {
    name: String,
    id: String
}

type ID = String;
type MessageMap = Arc<Mutex<HashMap<ID, Game>>>;

use ws::listen;
use ws::{Error, Handler, Handshake, CloseCode, Sender, Result, Message};
use env_logger;
const GAME_ID: &str = "abcde";

struct Server {
    out: Sender,
    games: MessageMap,
    outputs: Arc<Mutex<Vec<Sender>>>,
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
        // Add new output to outputs
        self.outputs.lock().unwrap().push(self.out.clone());

        let mut hashmap = self.games.lock().unwrap();
        if !hashmap.contains_key(GAME_ID) {
            println!("found key! {}", GAME_ID);
            let mut rng = thread_rng();
            let game = Game {
                name: GAME_ID.to_string(),
                state: BOARDS.choose(&mut rng).unwrap().to_string(),
                players: Vec::new()
            };
            hashmap.insert(GAME_ID.to_string(), game.clone());
        }
        let game = hashmap.get(GAME_ID).unwrap();
        println!("new connection: {}", self.out.connection_id());
        
        self.out.send(Message::text(json!({
            "state": &game.state,
            "gameName": &game.name
        }).to_string()))?;

        self.update_players(&game)?;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client says {}", msg);

        if let Message::Text(txt) = msg.clone() {
            let v: serde_json::Value = serde_json::from_str(&txt).unwrap();
            
            if &v["type"] == "send_digit" {
                let data: SendDigitMessage = serde_json::from_str(&txt).unwrap();

                let mut hashmap = self.games.lock().unwrap();
                let game = hashmap.get_mut(GAME_ID).unwrap();
                game.update_digit(data.icol, data.irow, data.digit);

                return self.send_everyone(Message::text(json!({
                    "state": &game.state,
                    "gameName": &game.name
                }).to_string()));
            } else if &v["type"] == "connect" {
                let mut hashmap = self.games.lock().unwrap();
                let game = hashmap.get_mut(GAME_ID).unwrap();

                let id = Uuid::new_v4().to_string();
                let name: String = String::from(v["name"].as_str().unwrap());
                game.add_player(Player {
                    name: name.clone(),
                    id: id.clone(),
                });

                self.out.send(Message::text(json!({
                    "type": "welcome",
                    "playerId": id,
                    "name": name.clone()
                }).to_string()))?;

                self.update_players(&game)?;
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
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

impl Server {
    fn send_everyone(&self, msg: Message) -> Result<()> {
        for out in self.outputs.lock().unwrap().iter() {
            out.send(msg.clone())?;
        }
        Ok(())
    }

    fn update_players(&self, game: &Game) -> Result<()> {
        let response = json!({
            "type": "update_players",
            "players": &game.players
        });
        self.send_everyone(Message::text(response.to_string()))?;
        Ok(())
    }
}

fn main() {
    env_logger::init();
    let map: MessageMap = Arc::new(Mutex::new(HashMap::<ID, Game>::new()));
    let output_counter = Arc::new(Mutex::new(Vec::new()));
    listen("127.0.0.1:3012", |out| { Server { outputs: output_counter.clone(), out: out, games: map.clone() } }).unwrap()
}