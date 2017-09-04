#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Json;
use std::process::Command;

#[derive(Deserialize)]
struct Task {
    command: String,
    complete: bool
}

#[post("/start", data = "<task>")]
fn start(task: Json<Task>) -> String {
    std::str::from_utf8(&Command::new(&task.command)
        .output()
        .expect("failed to execute process").stdout).unwrap().into()

}

fn main() {
    rocket::ignite().mount("/", routes![start]).launch();
}