#![allow(dead_code)]

pub struct CurrencyData {
    code: String,
    name: String,
}

pub enum FiatCurrency {
    Usd,
}

pub enum CryptoCurrency {
    Btc,
    Usdt,
}

pub enum Currency {
    FiatCurrency(FiatCurrency),
    CryptoCurrency(CryptoCurrency),
}

fn get_data_for_fiat_currency(currency: Currency) -> CurrencyData {
    match currency {
        Currency::FiatCurrency(FiatCurrency::Usd) => CurrencyData {
            code: String::from("USD"),
            name: String::from("US Dollar"),
        },
        Currency::CryptoCurrency(CryptoCurrency::Usdt) => CurrencyData {
            code: String::from("USDT"),
            name: String::from("Tether"),
        },
        Currency::CryptoCurrency(CryptoCurrency::Btc) => CurrencyData {
            code: String::from("BTC"),
            name: String::from("Bitcoin"),
        },
    }
}
