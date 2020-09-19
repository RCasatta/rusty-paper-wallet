use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::{secp256k1, Address, Network};
use qrcode::render::svg;
use qrcode::QrCode;
use rand::Rng;
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let secp = secp256k1::Secp256k1::signing_only();
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let extended_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;
    let private_key = extended_key.private_key;
    let public_key = private_key.public_key(&secp);
    let address = Address::p2wpkh(&public_key, Network::Bitcoin)?.to_string();
    let wif = private_key.to_wif();
    println!("wif {}", wif);
    println!("p2wpkh {}", address);

    let wif_qr_svg = create_svg_qr(&wif)?;
    let address_qr_svg = create_svg_qr(&address.to_uppercase())?;

    let page = format!(
        include_str!("template.html"),
        address, address_qr_svg, wif, wif_qr_svg
    );
    let file_name = format!("{}.html", address);

    println!("writing {}", &file_name);
    let mut file = std::fs::File::create(file_name)?;
    file.write_all(page.as_bytes())?;

    Ok(())
}

fn create_svg_qr(message: &str) -> Result<String> {
    let qr = QrCode::new(message.as_bytes())?;
    Ok(qr
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build())
}
