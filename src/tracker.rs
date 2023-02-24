extern crate anyhow;

use rand::Rng;
// use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use anyhow::{Result, anyhow};
use tokio::net::UdpSocket;
// use log::debug;
use url::Url;

use crate::torrent::{Torrent};
use crate::config;

/*
 * represent the init connection request
 */
#[derive(Serialize,Deserialize)]
struct TrackerConnect {
    connection_id: i64,
    action: i32,
    transaction_id: i32,
}

impl TrackerConnect {

    fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
        connection_id : 0x41727101980, // default
        action : 0, // for connection request
        transaction_id : rng.gen(),
        }
    }
}
/*
 * generate own peer id
 * random 20 byte
 */
fn get_peer_id() -> String {
    let mut rng = rand::thread_rng();
    let vals: Vec<u8> = (0..20).map(|_| rng.gen_range(0,255)).collect();
    hex::encode(vals)
}

pub async fn connect_to_tracker(torrent_data: &Torrent) -> Result<()> {
    let url = &torrent_data.announce_list.as_ref().unwrap()[8][0];
    let mut buffer: [u8; 1024] = [0; 1024];

    // build the parameters for the request
    let params = vec![
        ("info_hash", torrent_data.get_hashed_info()),
        ("peer_id", get_peer_id()),
        ("port", config::LISTEN_PORT.to_string()),
        ("uploaded", 0.to_string()),
        ("downloaded", 0.to_string()),
        ("left", torrent_data.get_download_size().to_string()),
    ];
    /*
    let params: String = params.into_iter().map(|x| {
        format!("{}={}", x.0, utf8_percent_encode(&x.1, NON_ALPHANUMERIC))
    }).collect::<Vec<String>>().join("&");
    */

    let sock_addr = Url::parse(url)?.socket_addrs(|| None)?;
    let srv = UdpSocket::bind("0.0.0.0:6969").await?;

    let mes = TrackerConnect::new();
    let tracker_bytes = bincode::serialize(&mes).unwrap();
    println!("addr : {}", sock_addr[0]);
    match srv.send_to(&tracker_bytes, sock_addr[0]).await {
        Err(err) => {
            panic!("problem with packet: {}", err);
        },
        Ok(sz) => {
            println!("yay {}", sz);
            ()}
        ,
    }
    srv.recv_from(&mut buffer).await?;
    Ok(())
}
