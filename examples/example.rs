use bitcoin_price;
fn main() {
    let coinbase_price = bitcoin_price::get_coinbase_price();
    let coinbase_price_string = coinbase_price.to_string();
    println!("coinbase: {}", coinbase_price_string);
    let kraken_price = bitcoin_price::get_kraken_price();
    let kraken_price_string = kraken_price.to_string();
    println!("kraken: {}", kraken_price_string);
    let average_exchange_price = bitcoin_price::get_average_exachange_spot_price();
    let average_exchange_price_string = average_exchange_price.to_string();
    println!("average_exchange_price: {}", average_exchange_price_string);
}
