use bitcoin::Network;
use log::error;
use rusty_paper_wallet::process;
use structopt::StructOpt;

/// Creates a paper wallet given a descriptor.
#[derive(StructOpt)]
struct Command {
    /// The descriptor defining the spending script of the paper wallet{n}
    /// Contains alias such as 'Alice' in place of keys that will be substituted with randomly generated keys.{n}
    /// For basic paper wallet use: 'wpkh(Alice)'
    descriptor: String,

    /// The network used, may be: bitcoin, testnet, regtest.
    #[structopt(short, default_value = "testnet")]
    network: Network,
}

fn main() {
    env_logger::init();
    let opt: Command = Command::from_args();

    match process(opt.descriptor, opt.network) {
        Ok(s) => println!("{}", s),
        Err(e) => error!("{}", e),
    }
}
