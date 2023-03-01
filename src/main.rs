
use grin_keychain;
use grin_keychain::keychain::ExtKeychain;
use grin_keychain::Keychain;
use rand::Rng;
use grin_wallet_libwallet::SlatepackAddress;
use grin_util::secp::key::SecretKey;
use grin_core::global;
use grin_core::global::ChainTypes;

fn main() {

    let mut rng = rand::thread_rng();
    
    let mut bytes = Vec::new();

    for _i in 0..24{
        let random_byte = rng.gen::<u8>();
        bytes.push(random_byte);
    }

    let seed = grin_keychain::mnemonic::from_entropy(&bytes).unwrap();

    let mut keychain = ExtKeychain::from_seed(&bytes, false).unwrap();

    let test = keychain.pub_root_key();

    // println!("{}", seed);
    // println!("{:?}", test);

    grin_core::global::set_local_chain_type(ChainTypes::Mainnet);


    let test1 = SlatepackAddress::random();

    println!("{}", test1);

}
