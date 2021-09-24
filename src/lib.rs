#![deny(missing_docs)]

//! # Rusty Paper Wallet
//!
//! Generates descriptor-based bitcoin paper wallet offline.

use crate::error::Error;
use crate::html::{paper_wallets, to_data_url, WalletData};
use bitcoin::secp256k1::{Secp256k1, Signing};
use bitcoin::{self, secp256k1, Address, Network, PublicKey};
use log::debug;
use miniscript::bitcoin::PrivateKey;
use miniscript::{self, Descriptor, DescriptorTrait, MiniscriptKey, TranslatePk};
use std::collections::BTreeMap;

mod error;
mod html;

/// Result type with implicit library Error
pub type Result<T> = std::result::Result<T, Error>;

/// Contains private key in WIF format and the relative public key in HEX format
#[derive(Debug)]
pub struct WifAndHexPub {
    wif: String,
    hex_pub: String,
}

/// Process descriptor and network and returns an html page encoded in a data-url which is pasteable
/// in a browser
pub fn process(descriptor: String, network: Network) -> Result<String> {
    let descriptor_string: Descriptor<String> = descriptor.parse()?;
    debug!("descriptor_string: {}", descriptor_string);

    let (keys, address) = create_key_pairs_and_address(&descriptor_string, network)?;

    let wallet_data = create_wallet_data(address, keys, &descriptor_string)?;

    let html = paper_wallets(&wallet_data)?;

    Ok(to_data_url(&html, "text/html"))
}

/// Creates a random key pair for the given alias and returns the public part.
/// While doing so it insert the WIF and the Public hex in the given `keys_map`
#[allow(clippy::unnecessary_wraps)] // avoid explicit notation on the closure
fn alias_to_key<T: Signing>(
    alias: &str,
    keys_map: &mut BTreeMap<String, WifAndHexPub>,
    network: Network,
    secp: &Secp256k1<T>,
) -> Result<PublicKey> {
    let key = secp256k1::SecretKey::new(&mut bitcoin::secp256k1::rand::thread_rng());
    let sk = PrivateKey {
        compressed: true,
        network,
        key,
    };
    let key = PublicKey::from_private_key(secp, &sk);
    keys_map.insert(
        alias.to_string(),
        WifAndHexPub {
            wif: sk.to_wif(),
            hex_pub: key.to_string(),
        },
    );

    Ok(key)
}

/// Creates a key pair for every alias in the given descriptor,
/// Creates another descriptor replacing alias with the relative public key, so that we can compute the address.
pub fn create_key_pairs_and_address(
    descriptor_string: &Descriptor<String>,
    network: Network,
) -> Result<(BTreeMap<String, WifAndHexPub>, Address)> {
    let secp = secp256k1::Secp256k1::signing_only();
    let mut keys = BTreeMap::new();
    let mut keys_2 = BTreeMap::new();

    let descriptor_keys: Descriptor<PublicKey> = descriptor_string.translate_pk(
        |alias| alias_to_key(alias, &mut keys, network, &secp),
        |alias| Ok(alias_to_key(alias, &mut keys_2, network, &secp)?.to_pubkeyhash()),
    )?;
    keys.extend(keys_2);

    debug!("descriptor_keys: {}", descriptor_keys);
    debug!("key_map: {:?}", keys);
    let address = descriptor_keys.address(network)?;
    debug!("address: {}", address.to_string());
    Ok((keys, address))
}

/// Returns the element in `legend` with key `alias`, returning an error if absent
#[allow(clippy::unnecessary_wraps)] // avoid explicit notation on the closure
fn alias_to_wif_or_pub(alias: &str, legend: &BTreeMap<&String, String>) -> Result<String> {
    let alias = alias.to_string();
    legend
        .get(&alias)
        .cloned()
        .ok_or(Error::MissingMappedKey(alias))
}

/// Creates data for every single paper wallet (which is different according to the relative owner)
fn create_wallet_data(
    address: Address,
    keys: BTreeMap<String, WifAndHexPub>,
    descriptor_alias: &Descriptor<String>,
) -> Result<Vec<WalletData>> {
    let descriptor_alias_string = descriptor_alias
        .to_string()
        .split('#')
        .next()
        .map(|s| s.to_string())
        .unwrap();

    let mut results = vec![];
    for alias in keys.keys() {
        let legend = keys
            .iter()
            .map(|(alias_internal, wif_and_pub)| {
                if alias == alias_internal {
                    (alias_internal, wif_and_pub.wif.clone())
                } else {
                    (alias_internal, wif_and_pub.hex_pub.clone())
                }
            })
            .collect::<BTreeMap<_, _>>();
        debug!("legend: {:?}", legend);

        let descriptor_qr = descriptor_alias
            .translate_pk(
                |alias| alias_to_wif_or_pub(alias, &legend),
                |alias| Ok(alias_to_wif_or_pub(alias, &legend)?.to_pubkeyhash()),
            )?
            .to_string();
        let checksum = descriptor_qr
            .split('#')
            .nth(1)
            .ok_or(Error::MissingChecksum)?;

        let wallet_data = WalletData {
            alias: alias.clone(),
            address: address.to_string(),
            address_qr: address.to_qr_uri(),
            descriptor_alias: format!("{}#{}", descriptor_alias_string, checksum),
            legend_rows: legend
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect(),
            descriptor_qr,
        };
        results.push(wallet_data);
    }

    Ok(results)
}
