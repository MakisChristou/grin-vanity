use clap::Parser;
use grin_core::global;
use grin_core::global::ChainTypes;
use grin_keychain;
use grin_keychain::keychain::ExtKeychain;
use grin_keychain::Identifier;
use grin_keychain::Keychain;
use grin_util::ToHex;
use grin_wallet_libwallet;
use grin_wallet_libwallet::SlatepackAddress;
use rand::Rng;
use std::process;
use std::thread;
use std::time::Instant;

use ed25519_dalek::PublicKey as edDalekPublicKey;
use ed25519_dalek::SecretKey as edDalekSecretKey;

mod args;
use args::Args;

// Measures how many s elapsed since the given instant
fn time_since(instant: Instant) -> f64 {
    return (instant.elapsed().as_nanos() as f64) / 1_000_000_000f64;
}

fn main() {
    let args = Args::parse();

    // Chosen or default settings
    println!("Searching for pattern {}", args.pattern);
    println!("Using {} threads", args.threads);

    if !args.pattern.starts_with("grin1") {
        println!("Pattern needs to start with grin1");
        process::exit(0x1);
    } else if args.pattern[5..]
        .to_string()
        .chars()
        .any(|c| c == '1' || c == 'i' || c == 'o' || c == 'b')
    {
        println!("Invalid pattern");
        println!("Valid characters are: acdefghjklmnpqrstuvwxyz023456789");
        std::process::exit(1);
    }

    let mut handles = Vec::new();

    // Spawn worker threads
    for thread_id in 0..args.threads {
        let pattern = args.pattern.clone();
        let refresh_interval = args.interval.clone();
        let parent_key_id = Identifier::from_bytes(&vec![
            02, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ]);

        let t = thread::spawn(move || {
            let mut i = 0;
            let start_time = Instant::now();
            let mut stats_timer = Instant::now();
            grin_core::global::set_local_chain_type(ChainTypes::Mainnet);

            loop {
                let loop_time = Instant::now();
                let bytes: [u8; 32] = rand::thread_rng().gen();

                // From seed
                let keychain = ExtKeychain::from_seed(&bytes, false).unwrap();
                let sec_addr_key = grin_wallet_libwallet::address::address_from_derivation_path(
                    &keychain,
                    &parent_key_id,
                    0,
                )
                .unwrap();
                let slatepack_address = SlatepackAddress::try_from(&sec_addr_key).unwrap();

                // From raw private key
                // let pub_key =
                //     edDalekPublicKey::from(&edDalekSecretKey::from_bytes(&bytes).unwrap());
                // let address = SlatepackAddress::new(&pub_key);

                if thread_id == 0 && time_since(stats_timer) > refresh_interval as f64 {
                    let pattern_length = pattern.len() - 5;
                    let num_of_patterns = 33_u64.pow(pattern_length as u32);
                    let iteration_time = time_since(loop_time);
                    let keys_per_second = (1. / iteration_time) * args.threads as f64;
                    let eta = (iteration_time * num_of_patterns as f64) / args.threads as f64;

                    print!("{:.2} keys/s ", keys_per_second);

                    if eta < 60. {
                        println!("eta: {:.2}s", eta as usize);
                    } else if eta < 3600. {
                        println!("eta: {:.2}min", eta / 60.);
                    } else if eta < 86400. {
                        println!("eta: {:.2}h", eta / 3600.);
                    } else if eta < 2073600. {
                        println!("eta: {:.2}d", eta / 86400.);
                    } else {
                        println!("eta: {:.2}y", eta / 2073600.);
                    }
                    stats_timer = Instant::now();
                }

                if slatepack_address.to_string().starts_with(&pattern) {
                    println!(
                        "\nFound address: {} \nWith Seed:     {} \n{} keys in {} seconds",
                        slatepack_address.to_string(),
                        grin_keychain::mnemonic::from_entropy(&bytes).unwrap(),
                        i * args.threads,
                        time_since(start_time)
                    );
                    process::exit(0x0);
                }
                // if address.to_string().starts_with(&pattern) {
                //     println!(
                //         "\nFound address: {} \nPrivate Key:   0x{} \n{} keys in {} seconds",
                //         address.to_string(),
                //         edDalekSecretKey::from_bytes(&bytes)
                //             .unwrap()
                //             .to_hex()
                //             .to_string(),
                //         i,
                //         time_since(start_time)
                //     );
                //     process::exit(0x0);
                // }
                i += 1;
            }
        });
        handles.push(t);
    }

    // Wait for threads to finish
    for handle in handles {
        handle.join().expect("Error joining worker thread");
    }
}
