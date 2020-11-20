extern crate serde_bencode;
extern crate serde;
extern crate hex;

use serde_bencode::{de,ser};
use std::io::Read;
use serde_bytes::ByteBuf;
use std::fs::File;
use sha1::{Sha1, Digest};


#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize, Serialize)]
struct TorrentFile {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Info {
    name: String,
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)] files: Option<Vec<TorrentFile>>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="root hash")]
    root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    info: Info,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename="announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename="creation date")]
    creation_date: Option<i64>,
    #[serde(rename="comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename="created by")]
    created_by: Option<String>,
}

pub fn render_torrent(torrent: &Torrent) {
    println!("name:\t\t{}", torrent.info.name);
    println!("announce:\t{:?}", torrent.announce);
    println!("nodes:\t\t{:?}", torrent.nodes);
    if let &Some(ref al) = &torrent.announce_list {
        for a in al {
            println!("announce list:\t{}", a[0]);
        }
    }
    println!("httpseeds:\t{:?}", torrent.httpseeds);
    println!("creation date:\t{:?}", torrent.creation_date);
    println!("comment:\t{:?}", torrent.comment);
    println!("created by:\t{:?}", torrent.created_by);
    println!("encoding:\t{:?}", torrent.encoding);
    println!("piece length:\t{:?}", torrent.info.piece_length);
    println!("private:\t{:?}", torrent.info.private);
    println!("root hash:\t{:?}", torrent.info.root_hash);
    println!("md5sum:\t\t{:?}", torrent.info.md5sum);
    println!("path:\t\t{:?}", torrent.info.path);
    if let &Some(ref files) = &torrent.info.files {
        for f in files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
            println!("file md5sum:\t{:?}", f.md5sum);
        }
    }
}

impl Torrent {
    pub fn new(file_path: &String) -> Option<Torrent> {
        let mut buffer = Vec::new();

        let mut file_handle = File::open(file_path).unwrap();
        match file_handle.read_to_end(&mut buffer) {
            Ok(_) => Some(de::from_bytes::<Torrent>(&buffer).unwrap()),
            Err(e) => {
                eprintln!("Error {:?}", e);
                None
            }
        }
    }

    /*
     * return the total size to download from the info struct
     */
    pub fn get_download_size(&self) -> i64 {
        let mut total_size: i64 = 0;
        for file in self.info.files.as_ref().unwrap().iter() {
            total_size += file.length;
        }
        total_size
    }

    /*
     * Return sha1 of bencoded info dict
     */
    pub fn get_hashed_info(&self) -> String {
        let bytes = ser::to_bytes::<Info>(&self.info).unwrap();

        let mut hasher = Sha1::new();
        hasher.update(bytes);
        hex::encode(hasher.finalize().to_vec())
    }
}
