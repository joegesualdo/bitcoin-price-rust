use bitcoin_price;
fn main() {
    let coinbase_price = bitcoin_price::get_coinbase_price();
    let coinbase_price_string = coinbase_price.to_string();
    println!("{}", coinbase_price_string);
    let kraken_price = bitcoin_price::get_kraken_price();
    let kraken_price_string = kraken_price.to_string();
    println!("{}", kraken_price_string);
}
