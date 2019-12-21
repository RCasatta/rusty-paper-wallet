use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::Address;
use bitcoin::Network;
use qrcode::render::svg;
use qrcode::QrCode;
use rand::Rng;
use secp256k1::Secp256k1;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let secp = &Secp256k1::signing_only();
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let extended_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();
    let private_key = extended_key.private_key;
    let public_key = private_key.public_key(&secp);
    let address = Address::p2wpkh(&public_key, Network::Bitcoin).to_string();
    let wif = private_key.to_wif();
    println!("wif {}", wif);
    println!("p2wpkh {}", address);

    let wif_qr = QrCode::new(wif.as_bytes()).unwrap();
    let address_qr = QrCode::new(address.to_uppercase().as_bytes()).unwrap();

    let wif_qr_svg = wif_qr
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    let address_qr_svg = address_qr
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    let page = format!(
        include_str!("template.html"),
        address, address_qr_svg, wif, wif_qr_svg
    );
    let file_name = format!("{}.html", address.to_string());

    println!("writing {}", &file_name);
    let mut file = File::create(file_name).unwrap();
    file.write_all(page.as_bytes()).unwrap();
}
