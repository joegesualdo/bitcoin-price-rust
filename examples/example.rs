use bitcoin_price;
fn main() {
    let price = bitcoin_price::get_coinbase_price();
    let price_string = price.to_string();
    println!("{}", price_string);
}
