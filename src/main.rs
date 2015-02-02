
use std::string;
use std::result;

extern crate sodiumoxide;

use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::asymmetricbox;

mod base32;
mod e3x;

fn main() {
    sodiumoxide::init();
    let hash::sha256::Digest(d) = hash::sha256::hash("Hello, world!".as_bytes());

    let rd = base32::encode(&d);
    println!("{}", rd.unwrap());
    println!("{}", 'A' as u8);

    let (asymmetricbox::curve25519xsalsa20poly1305::PublicKey(k1), k2) = asymmetricbox::curve25519xsalsa20poly1305::gen_keypair();

    let r = base32::encode(&k1);
    println!("{}", r.unwrap());

    // --
    let id = [0x1a];
    let hash::sha256::Digest(d1) = hash::sha256::hash(&id);

    /*    js hash = sha256(0x1a) hash = sha256(hash + base32decode(")) hash = sha256(hash + 0x3a) hash = sha256(hash + base32decode("ckczcg2fq5hhaksfqgnm44xzheku6t7c4zksbd3dr4wffdvvem6q")) print base32encode(hash) "27ywx5e5ylzxfzxrhptowvwntqrd3jhksyxrfkzi6jfn64d3lwxa"*/
}



