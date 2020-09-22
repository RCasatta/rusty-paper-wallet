use bitcoin::{secp256k1, Address, Network, PrivateKey, PublicKey};
use qrcode::{render::svg, QrCode};
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let key = secp256k1::SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key,
    };
    let secp = secp256k1::Secp256k1::signing_only();
    let public_key = private_key.public_key(&secp);
    let public_key_check = private_key.public_key(&secp);
    assert_eq!(public_key, public_key_check, "Bip flip!");

    let address_type = std::env::var("ADDRESS_TYPE").unwrap_or("p2wpkh".to_string());
    let address = create_address(&public_key, &address_type)?;
    let address_check = create_address(&public_key, &address_type)?;
    assert_eq!(address, address_check, "Bip flip!");

    let optionally_uppercased = if address == "p2wpkh" {
        address.to_uppercase()
    } else {
        address.clone()
    };
    let wif = private_key.to_wif();
    println!("wif {}", wif);
    println!("{} {}", address_type, address);

    let wif_qr_svg = create_svg_qr(&wif)?;
    let address_qr_svg = create_svg_qr(&optionally_uppercased)?;

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

fn create_address(public_key: &PublicKey, address_type: &String) -> Result<String> {
    Ok(match address_type.as_str() {
        "p2wpkh" => Address::p2wpkh(&public_key, Network::Bitcoin)?.to_string(),
        "p2pkh" => Address::p2pkh(&public_key, Network::Bitcoin).to_string(),
        "p2shwpkh" => Address::p2shwpkh(&public_key, Network::Bitcoin)?.to_string(),
        _ => panic!("invalid ADDRESS_TYPE"),
    })
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
