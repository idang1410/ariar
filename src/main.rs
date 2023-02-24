#[macro_use]
extern crate serde_derive;
extern crate serde_bytes;
extern crate log;

mod tracker;
mod torrent;
mod consts;
mod config;

use clap::{Arg, App};
use std::error::Error;

use torrent::{Torrent, render_torrent};
use tracker::connect_to_tracker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let params = get_params();
    let torrent_data = Torrent::new(&params).unwrap();
    render_torrent(&torrent_data);
    let return_value = connect_to_tracker(&torrent_data).await?;
    println!("the params {}", params);
    Ok(return_value)
}

/*
 * handle the command line arguments
 */
fn get_params() -> String {
    let matches = App::new("ariar")
        .version("0.1")
        .about("download manager written in rust")
        .author("idang1410");
    let param = Arg::with_name("file")
        .long("file")
        .short("f")
        .value_name("FILE")
        .help("torrent file")
        .required(true);

    let matches = matches.arg(param);
    let matches = matches.get_matches();
    String::from(matches.value_of("file").unwrap().clone())
}

