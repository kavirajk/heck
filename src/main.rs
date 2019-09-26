mod config;
mod storage;
mod server;

use clap::{Arg, App};

use config::load_config;
use server::Server;

fn main() {
    let matches = App::new("Heck server")
        .version("0.01")
        .author("Kaviraj<kavirajkanagaraj@gmail.com>")
        .about("A Distributed Health Check Server")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("sets a custom config file")
             .takes_value(true)
        ).get_matches();

    let cfg = load_config(
        matches.value_of("config").expect("please provide config file")
    ).expect("failed to load config file");

    let storage = redis::Client::open("redis://127.0.0.1").expect("failed to get redis client");

    let server = Server::new(&cfg, storage);

    println!("heck server starting...");

    server.start();
}

/* TODO
1. Parse server config from yaml - DONE
2. add storage layer via redis
3. Use object pool pattern for redis connection pool
4. Add Client CLI to query from storage. 1. Servers that are live, or dead 2. Status of specific server
5. Custom deserialize time::Duration into from the form "12h13m" -> time::Duration
6. Add proper struct fields with reference rather than cloning
*/
