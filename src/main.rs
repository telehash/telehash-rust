
#![feature(macro_rules)]

extern crate sodiumoxide;

use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::asymmetricbox;

mod base32;
mod e3x;

#[deriving(FromPrimitive)]
enum Foo {
    Zeroth,
    First,
    Second
}

fn main() {
    sodiumoxide::init();
    let hash::sha256::Digest(d) = hash::sha256::hash("Hello, world!".as_bytes());
    println!("{}", base32::encode(&d));
    println!("{}", 'A' as u8);

    let (asymmetricbox::curve25519xsalsa20poly1305::PublicKey(k1), k2) = asymmetricbox::curve25519xsalsa20poly1305::gen_keypair();
    println!("{}", base32::encode(&k1));

    // --
    let id = [0x1a];
    let hash::sha256::Digest(d1) = hash::sha256::hash(&id);

    let f = Foo::Second;
    println!("{}", f as uint);
    let id = Foo::from_u8(1);
    println!("{}", id);

    /*    js hash = sha256(0x1a) hash = sha256(hash + base32decode(")) hash = sha256(hash + 0x3a) hash = sha256(hash + base32decode("ckczcg2fq5hhaksfqgnm44xzheku6t7c4zksbd3dr4wffdvvem6q")) print base32encode(hash) "27ywx5e5ylzxfzxrhptowvwntqrd3jhksyxrfkzi6jfn64d3lwxa"*/
}



