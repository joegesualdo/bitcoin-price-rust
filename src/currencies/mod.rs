struct CurrencyData {
    code: String,
    name: String,
}

pub enum FiatCurrency {
    USD,
}

pub enum CryptoCurrency {
    BTC,
}

pub enum Currency {
    FiatCurrency(FiatCurrency),
    CryptoCurrency(CryptoCurrency)
}

fn get_data_for_fiat_currency(currency: Currency) -> CurrencyData {
    match currency {
        Currency::FiatCurrency(FiatCurrency::USD) => CurrencyData {
            code: String::from("USD"),
            name: String::from("US Dollar"),
        },
        Currency::CryptoCurrency(CryptoCurrency::BTC) => CurrencyData {
            code: String::from("BTC"),
            name: String::from("Bitcoin"),
        },
    }
}