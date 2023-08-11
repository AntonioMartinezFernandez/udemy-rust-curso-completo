/**************************************************************************************
 *
 *        CRYPTO CURRENCY PRICES WITH WEBASSEMBLY PROJECT
 *
 *    - Build project with 'wasm-pack build --release --target web'
 *    - Serve the file crypto-prices-wasm/webapp/index.html (i.e. with Live Server)
 *
 *************************************************************************************/

use serde::Deserialize;
// use serde_json::{Map, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }

#[wasm_bindgen]
pub async fn get_coin_price(coin_name: String) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let api_url = format!("https://api.coingecko.com/api/v3/coins/{coin_name}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false");

    let request = Request::new_with_str_and_init(&api_url, &opts)?;
    let window = web_sys::window().unwrap();
    let api_response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = api_response.dyn_into().unwrap();
    let response_text = JsFuture::from(response.text()?).await?.as_string().unwrap();

    let coin_price = parse_response_text(&response_text);
    Ok(coin_price)
}

fn parse_response_text(api_response_text: &String) -> String {
    // let json: Map<String, Value> = serde_json::from_str(&api_response_text).unwrap();
    // let price = json["market_data"]["current_price"]["eur"].to_string()

    let coin_data: CoinData = serde_json::from_str(&api_response_text).unwrap();
    coin_data.market_data.current_price.eur.to_string()
}

#[derive(Deserialize)]
struct CoinData {
    // id: String,
    // name: String,
    // symbol: String,
    // image: Map<String, Value>,
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    current_price: CurrentPrice,
}

#[derive(Deserialize)]
struct CurrentPrice {
    // usd: String,
    eur: f32,
}
