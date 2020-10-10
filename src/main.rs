extern crate clap;

use clap::{Arg, App};

fn main() {
    let params = get_params();
    println!("the params {}", params);
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

