use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::Network;
use rand::Rng;
use secp256k1::Secp256k1;
use bitcoin::Address;

fn main() {
    let secp = &Secp256k1::signing_only();
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let extended_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();
    let private_key = extended_key.private_key;
    let public_key = private_key.public_key(&secp);
    let address = Address::p2wpkh(&public_key, Network::Bitcoin);
    println!("wif {}", private_key.to_wif());
    println!("p2wpkh {}", address);
}
