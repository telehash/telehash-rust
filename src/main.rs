
#![feature(core)]
#![feature(collections)]

extern crate sodiumoxide;

use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::asymmetricbox;

mod base32;

fn main() {
    sodiumoxide::init();
    let hash::sha256::Digest(d) = hash::sha256::hash("Hello, world!".as_bytes());

    let rd = base32::encode(&d);
    println!("{}", rd);
    println!("{}", 'A' as u8);

    // Generate the key pair
    let (asymmetricbox::curve25519xsalsa20poly1305::PublicKey(pubk), _privk) =
        asymmetricbox::curve25519xsalsa20poly1305::gen_keypair();

    // Generate intermediate key
    let hash::sha256::Digest(intermedk) = hash::sha256::hash(&pubk);

    // Push rollup and intermediate into a vector
    let mut rollup: Vec<u8> = Vec::new();
    rollup.push(0x1a);
    rollup.push_all(&intermedk);

    // Generate digest and then base32 encode
    let hash::sha256::Digest(finalk) = hash::sha256::hash(rollup.as_slice());
    let kd = base32::encode(&finalk);
    println!("Key: {}", kd);

    /*    js hash = sha256(0x1a) hash = sha256(hash + base32decode(")) hash = sha256(hash + 0x3a) hash = sha256(hash + base32decode("ckczcg2fq5hhaksfqgnm44xzheku6t7c4zksbd3dr4wffdvvem6q")) print base32encode(hash) "27ywx5e5ylzxfzxrhptowvwntqrd3jhksyxrfkzi6jfn64d3lwxa"*/
}



