use crate::Result;
use maud::{html, PreEscaped};
use qr_code::QrCode;
use std::io::Cursor;

const CSS: &str = include_str!("html.css");

#[derive(Debug, Clone)]
pub struct WalletData {
    /// Alias name of the owner of the paper wallet, shown in public part
    pub alias: String,
    /// Address of the paper wallet
    pub address: String,
    /// Address of the paper wallet, uppercase if bech32 to improve QR code
    pub address_qr: String,
    /// Descriptor containing aliases
    pub descriptor_alias: String,
    /// Legend containing Alias -> Key correspondence
    pub legend_rows: Vec<String>,
    /// Descriptor containing keys
    pub descriptor_qr: String,
}

/// Converts `input` in base64 and returns a data url
pub fn to_data_url<T: AsRef<[u8]>>(input: T, content_type: &str) -> String {
    let base64 = base64::encode(input.as_ref());
    format!("data:{};base64,{}", content_type, base64)
}

/// Creates QR containing `message` and encode it in data url
fn create_bmp_base64_qr(message: &str) -> Result<String> {
    let qr = QrCode::new(message.as_bytes())?;

    // The `.mul(3)` with pixelated rescale shouldn't be needed, however, some printers doesn't
    // recognize it resulting in a blurry image, starting with a bigger image mostly prevents the
    // issue at the cost of a bigger image size.
    let bmp = qr.to_bmp().mul(3)?;

    let mut cursor = Cursor::new(vec![]);
    bmp.write(&mut cursor).unwrap();
    Ok(to_data_url(cursor.into_inner(), "image/bmp"))
}

/// Creates the inner html of a single paper wallet
fn inner(data: &WalletData) -> Result<PreEscaped<String>> {
    let public_qr = create_bmp_base64_qr(&data.address_qr)?;
    let private_qr = create_bmp_base64_qr(&data.descriptor_qr)?;

    let single = html! {
        div class="single" {
            div {
                div class="break-word" {
                    span class="bold" {
                        (&data.alias)
                    }
                    br;
                    (&data.address)
                }
                div class="center" {
                    img class="qr" src=(public_qr) { }
                }
            }

            div class="black" {}

            div {
                div class="break-word" {
                    div class="pad" { (&data.descriptor_alias) }
                    @for row in &data.legend_rows { div class="pad" { (*row) }  }
                }
                div class="center" {
                    img class="qr" src=(private_qr) { }
                }
            }
        }
    };

    Ok(single)
}

/// Returns the html page containing the given `paper_wallets`
pub fn paper_wallets(paper_wallets: &[WalletData]) -> Result<String> {
    let mut paper_wallets_html = vec![];
    for paper_wallet in paper_wallets {
        paper_wallets_html.push(inner(paper_wallet).unwrap());
    }
    let css = CSS.replace("\n", "").replace("  ", " ").replace("  ", " ");
    let html = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="UTF-8" {}
                title { "Bitcoin Paper Wallet" }
                style { (css) }
            }
            body {
                @for paper_wallet in &paper_wallets_html { (*paper_wallet)  }
            }
        }
    };

    Ok(html.into_string())
}

#[cfg(test)]
mod test {
    use crate::html::{inner, paper_wallets, to_data_url, WalletData};

    #[test]
    fn test_html() {
        let data = WalletData {
            alias: "Riccardo".to_string(),
            address: "bc1qthxwqly0f3uyllhxezmyukrjwkxer6ezdlekhu".to_string(),
            address_qr: "BC1QTHXWQLY0F3UYLLHXEZMYUKRJWKXER6EZDLEKHU".to_string(),
            descriptor_alias: "wpkh(Riccardo)".to_string(),
            legend_rows: vec![
                "Riccardo: L2a7AaJEv2ef5UE25keeHNhfN45jMepMLS9dar2ChL68fJ4L8NfL".to_string(),
            ],
            descriptor_qr: "L2a7AaJEv2ef5UE25keeHNhfN45jMepMLS9dar2ChL68fJ4L8NfL".to_string(),
        };
        let inner_html = inner(&data).unwrap();
        assert!(inner_html.into_string().contains(&data.address));

        let paper_wallets_data = vec![data.clone(), data.clone()];
        let html = paper_wallets(&paper_wallets_data).unwrap();
        println!("{}", to_data_url(&html, "text/html"));
    }
}
