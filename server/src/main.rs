#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(Debug)]
struct Game {
    name: String
}

#[get("/<game>")]
fn index(game: String) -> Result<String> {
    let conn = Connection::open("games.db")?;

    let mut stmt = conn.prepare("SELECT name from games WHERE name == $1")?;

    let games = stmt.query_map(&[&game], |row| {
        Game {
            name: row.get(0),
        }
    })?;

    if (games.count() > 0) {
        return Ok(String::from("game exists!"));
    }

    conn.execute(
        "INSERT INTO games (name) values ($1)",
        &[&game],
    )?;

    return Ok(String::from("Created game"));
}

fn init_db() -> Result<()> {
    let mut conn = Connection::open("games.db")?;

    conn.execute(
        "create table if not exists games (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn main() {
    init_db();

    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

