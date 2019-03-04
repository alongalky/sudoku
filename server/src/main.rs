#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate rusqlite;
extern crate rand;
 
use rand::thread_rng;
use rand::seq::SliceRandom;

use rusqlite::{Connection, Result, NO_PARAMS};
mod boards;
use boards::BOARDS;

#[derive(Debug)]
struct Game {
    name: String,
    state: String
}

#[get("/<game_name>")]
fn index(game_name: String) -> Result<String> {
    let conn = Connection::open("games.db")?;

    let mut stmt = conn.prepare("SELECT name, state from games WHERE name == $1")?;

    let games: Vec<Game> = stmt.query_map(&[&game_name], |row| {
        Game {
            name: row.get(0),
            state: row.get(1)
        }
    })?.map(|row| row.unwrap()).collect();

    if games.len() > 0 {
        let game = &games[0];
        return Ok(format!("game {} exists. Its state is {}", game_name, game.state));
    }

    let mut rng = thread_rng();
    conn.execute(
        "INSERT INTO games (name, state) values ($1, $2)",
        &[&game_name, &BOARDS.choose(&mut rng).unwrap().to_string()],
    )?;

    return Ok(String::from("Created game"));
}

fn init_db() -> Result<()> {
    let conn = Connection::open("games.db")?;

    conn.execute(
        "create table if not exists games (
             id integer primary key,
             name text not null unique,
             state text not null unique
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn main() {
    match init_db() {
        Ok(()) => {
            rocket::ignite()
                .mount("/", routes![index])
                .launch();
        },
        Err(_err) => println!("Failed to initialize db")
    }
}

