#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket_contrib::json::Json;

extern crate rand;
 
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::sync::Mutex;
use std::collections::HashMap;

mod boards;
use boards::BOARDS;

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct Game {
    name: String,
    state: String
}

type ID = String;
type MessageMap = Mutex<HashMap<ID, Game>>;

#[get("/<game_name>")]
fn index(game_name: String, games: State<MessageMap>) -> Json<Game> {
    let mut hashmap = games.lock().unwrap();
    
    if hashmap.contains_key(&game_name) {
        return Json(hashmap.get(&game_name).unwrap().clone());
    } else {
        let mut rng = thread_rng();
        let game = Game {
            name: game_name.clone(),
            state: BOARDS.choose(&mut rng).unwrap().to_string()
        };
        hashmap.insert(game_name.clone(), game.clone());
        return Json(game);
    }
}

#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, Game>::new()))
        .launch();
}
