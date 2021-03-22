use std::fmt::{self, Formatter};

#[derive(Debug)]
pub enum Error {
    Qr(qr_code::types::QrError),
    Address(bitcoin::util::address::Error),
    Secp256k1(bitcoin::secp256k1::Error),
    Miniscript(miniscript::Error),
    Bmp(qr_code::bmp_monochrome::BmpError),
    InvalidAddressType,
    MissingChecksum,
    MissingMappedKey(String),
    OnlyPkh,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Qr(e) => write!(f, "{:?}", e),
            Error::Address(e) => write!(f, "{:?}", e),
            Error::Miniscript(e) => write!(f, "{:?}", e),
            Error::Secp256k1(e) => write!(f, "{:?}", e),
            Error::Bmp(e) => write!(f, "{:?}", e),
            Error::InvalidAddressType => write!(f, "Valid values: wpkh, wsh, pkh, shwpkh"),
            Error::MissingMappedKey(s) => write!(f, "Missing mapped key for alias {}", s),
            Error::OnlyPkh => write!(f, "Only *pkh address: wpkh, pkh, shwpkh"),
            Error::MissingChecksum => write!(f, "Missing checksum"),
        }
    }
}

macro_rules! impl_error {
    ( $from:ty, $to:ident ) => {
        impl std::convert::From<$from> for Error {
            fn from(err: $from) -> Self {
                Error::$to(err)
            }
        }
    };
}

impl_error!(bitcoin::util::address::Error, Address);
impl_error!(miniscript::Error, Miniscript);
impl_error!(bitcoin::secp256k1::Error, Secp256k1);
impl_error!(qr_code::types::QrError, Qr);
impl_error!(qr_code::bmp_monochrome::BmpError, Bmp);
