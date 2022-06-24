use bitcoin_price;
use coinbase_bitcoin;
fn main() {
    let coinbase_price = coinbase_bitcoin::get_price_data().spot;
    let coinbase_price_string = coinbase_price.to_string();
    println!("coinbase: {}", coinbase_price_string);
    let coinbase_buy_price = bitcoin_price::get_coinbase_buy_price();
    let coinbase_buy_price_string = coinbase_buy_price.to_string();
    println!("coinbase buy price: {}", coinbase_buy_price_string);
    let coinbase_sell_price = bitcoin_price::get_coinbase_sell_price();
    let coinbase_sell_price_string = coinbase_sell_price.to_string();
    println!("coinbase sell price: {}", coinbase_sell_price_string);
    let kraken_price = bitcoin_price::get_kraken_price();
    let kraken_price_string = kraken_price.to_string();
    println!("kraken: {}", kraken_price_string);
    let ftx_price = bitcoin_price::get_ftx_price();
    let ftx_price_string = ftx_price.to_string();
    println!("ftx: {}", ftx_price_string);
    let binance_price = bitcoin_price::get_binance_price();
    let binance_price_string = binance_price.to_string();
    println!("binance: {}", binance_price_string);
    let crypto_dot_com_price = bitcoin_price::get_crypto_dot_com_price();
    let crypto_dot_com_price_string= crypto_dot_com_price.to_string();
    println!("crypto.com: {}", crypto_dot_com_price_string);
    let average_exchange_price = bitcoin_price::get_average_exchange_spot_price();
    let average_exchange_price_string = average_exchange_price.to_string();
    println!("average_exchange_price: {}", average_exchange_price_string);
}
