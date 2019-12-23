# Rusty-paper-wallet

Generates bitcoin paper wallet offline in a single html page with p2wpkh address.

![paper-wallet](rusty-paper-wallet-example.png)

# Usage

Cut the dotted line, fold the private key over the black area, then fold a second time, plastify the paper.

# Running the software

Requires [rust](https://www.rust-lang.org/)

```
$ git clone https://github.com/RCasatta/rusty-paper-wallet
$ cd rusty-paper-wallet
$ cargo run
wif L4rHbEMQJPFn6GUVNZttgfb4HpEZbXeTf3xAWECJHmgqQ2utDhRx
p2wpkh bc1q6zm2qndz4qtq4h6hh802j3v5gggd4825lntffn
writing bc1q6zm2qndz4qtq4h6hh802j3v5gggd4825lntffn.html

```