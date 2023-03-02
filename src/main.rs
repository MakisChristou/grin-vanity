use grin_core::global;
use grin_core::global::ChainTypes;
use grin_keychain;
use grin_keychain::keychain::ExtKeychain;
use grin_keychain::Keychain;
use grin_util::ToHex;
use grin_wallet_libwallet;
use grin_wallet_libwallet::SlatepackAddress;
use rand::Rng;
use std::time::Instant;
use clap::Parser;
use std::process;
use std::thread;

use ed25519_dalek::PublicKey as edDalekPublicKey;
use ed25519_dalek::SecretKey as edDalekSecretKey;

// Measures how many s elapsed since the given instant
fn time_since(instant: Instant) -> f64 {
    return (instant.elapsed().as_nanos() as f64) / 1_000_000_000f64;
}

mod args;
use args::Args;

fn main() {

    let args = Args::parse();

    // Chosen or default settings
    println!("Searching for pattern {}", args.pattern);
    println!("Using {} threads", args.threads);

    if !args.pattern.starts_with("grin1") {
        println!("Pattern needs to start with grin1");
        process::exit(0x1);
    }

    // let mut rng = rand::thread_rng();

    // let mut bytes = Vec::new();

    // for _i in 0..24 {
    //     let random_byte = rng.gen::<u8>();
    //     bytes.push(random_byte);
    // }

    // let seed = grin_keychain::mnemonic::from_entropy(&bytes).unwrap();

    // let mut keychain = ExtKeychain::from_seed(&bytes, false).unwrap();

    // let test = keychain.pub_root_key().public_key;

    // println!("{}", seed);
    // println!("{:?}", test);


    grin_core::global::set_local_chain_type(ChainTypes::Mainnet);

    let mut i = 0;
    let start_time = Instant::now();

    loop {
        let loop_time = Instant::now();
        let bytes: [u8; 32] = rand::thread_rng().gen();
        let pub_key = edDalekPublicKey::from(&edDalekSecretKey::from_bytes(&bytes).unwrap());
        let address = SlatepackAddress::new(&pub_key);

        if i % 50000 == 0 {
            println!(
                "{} keys per second",
                1. / time_since(loop_time)
            );
        }

        if address.to_string().starts_with(&args.pattern) {
            println!(
                "\nFound address: {} \nPrivate Key:   0x{} \n{} keys in {} seconds",
                address.to_string(),
                edDalekSecretKey::from_bytes(&bytes).unwrap().to_hex().to_string(),
                i,
                time_since(start_time)
            );
            process::exit(0x0);
        }

        i += 1;
    }
}
