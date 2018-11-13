extern crate cqrs_todoql_psql;
extern crate clap;
extern crate env_logger;

use clap::{App, Arg};

use cqrs_todoql_psql::start_todo_server;

fn main() {
    env_logger::init();

    let app = App::new("todo")
        .arg(Arg::with_name("conn-str")
            .long("connection-string")
            .short("c")
            .takes_value(true)
            .help("Backend PostgreSQL connection string")
            .value_name("postgresql://user:pass@localhost:5433/db")
        );

    let matches = app.get_matches();

    let listening = start_todo_server(matches.value_of("conn-str").unwrap());

    println!("Now listening at {}", listening.socket);
    println!("Press Ctrl+C to quit");
}