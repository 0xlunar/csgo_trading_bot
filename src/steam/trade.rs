use std::collections::{HashMap, BTreeSet};
use std::iter::FromIterator;
use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
use dotenv;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use super::Inventory::UnauthorizedResponse;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TradeOfferData {
  newversion: bool,
  version: i32,
  pub me: OfferData,
  pub them: OfferData
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OfferData {
  pub assets: Vec<OfferAsset>,
  pub currency: Vec<String>,
  pub ready: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OfferAsset {
  pub appid: String,
  pub contextid: String,
  pub amount: String,
  pub assetid: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TradeOffer {
  sessionid: String,
  serverid: String,
  pub partner: String,
  pub tradeoffermessage: String,
  pub json_tradeoffer: TradeOfferData,
  captcha: String,
  pub trade_offer_create_params: TradeOfferCreateParams,
  //#[serde(skip)]
  trade_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TradeOfferForm {
  sessionid: String,
  serverid: String,
  pub partner: String,
  pub tradeoffermessage: String,
  pub json_tradeoffer: String,
  captcha: String,
  pub trade_offer_create_params: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TradeOfferCreateParams {
  pub trade_offer_access_token: String,
}

impl TradeOffer {
  pub fn new(trade_url: String) -> TradeOffer {

    let parsed_url: HashMap<String, String> = reqwest::Url::parse(&trade_url).unwrap().query_pairs().into_owned().collect();

    let partnerid = match parsed_url.get("partner") {
      Some(p) => p,
      None => panic!("Failed to get partner Id")
    };

    let access_token = match parsed_url.get("token") {
      Some(t) => t,
      None => panic!("Failed to get partner Id")
    };

    let steam_id: u64 = convert_parterid_to_steamid(partnerid);

    TradeOffer { sessionid: "".to_string(), serverid: "1".to_string(), partner: steam_id.to_string(), tradeoffermessage: String::new(), json_tradeoffer: TradeOfferData::new(), captcha: "".to_string(), trade_offer_create_params: TradeOfferCreateParams { trade_offer_access_token: access_token.to_owned() }, trade_url }
  }

  pub fn set_trade_message(&mut self, message: String) {
    self.tradeoffermessage = message;
  }

  pub fn add_self_item(&mut self, asset: OfferAsset) {
    self.json_tradeoffer.me.assets.push(asset)
  }

  pub fn add_self_items(&mut self, assets: Vec<OfferAsset>) {
    self.json_tradeoffer.me.assets.extend(assets)
  }

  pub fn add_partner_item(&mut self, asset: OfferAsset) {
    self.json_tradeoffer.them.assets.push(asset)
  }

  pub fn add_partner_items(&mut self, assets: Vec<OfferAsset>) {
    self.json_tradeoffer.them.assets.extend(assets)
  }

  pub fn remove_self_item(&mut self, assetid: String) {
    self.json_tradeoffer.me.assets.retain(|a| a.assetid != assetid)
  }

  pub fn remove_self_items(&mut self, assetids: Vec<String>) {
    let to_remove = BTreeSet::from_iter(assetids);
    self.json_tradeoffer.me.assets.retain(|a| !to_remove.contains(&a.assetid))
  }

  pub fn remove_partner_item(&mut self, assetid: String) {
    self.json_tradeoffer.them.assets.retain(|a| a.assetid != assetid)
  }

  pub fn remove_partner_items(&mut self, assetids: Vec<String>) {
    let to_remove = BTreeSet::from_iter(assetids);
    self.json_tradeoffer.them.assets.retain(|a| !to_remove.contains(&a.assetid))
  }

  pub fn toggle_self_ready(&mut self) {
    self.json_tradeoffer.me.ready = !self.json_tradeoffer.me.ready
  }

  pub fn toggle_partner_ready(&mut self) {
    self.json_tradeoffer.them.ready = !self.json_tradeoffer.them.ready
  }

  pub async fn send(&mut self, cookie: &String) -> Result<(),UnauthorizedResponse> {
    let client = Client::new();

    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut bytes = [0u8; 12];
    rng.fill_bytes(&mut bytes);
  
    let session_id = hex::encode(&bytes);
    self.sessionid = session_id.to_owned();
    
    let cookie = format!("{}sessionid={};", cookie, session_id);

    let json_data = serde_json::to_string(&self.json_tradeoffer).unwrap();
    let token_data = serde_json::to_string(&self.trade_offer_create_params).unwrap();

    let form_data = TradeOfferForm {
      serverid: self.serverid.to_owned(),
      sessionid: self.sessionid.to_owned(),
      partner: self.partner.to_owned(),
      tradeoffermessage: self.tradeoffermessage.to_owned(),
      json_tradeoffer: json_data,
      captcha: self.captcha.to_owned(),
      trade_offer_create_params: token_data,
    };

    let res = client.post("https://steamcommunity.com/tradeoffer/new/send")
      .header("Referer", &self.trade_url)
      .header("Cookie", cookie)
      .form(&form_data)
      .send().await.expect("Failed to send request");

    let status = res.status().to_owned();

    let text = res.text().await.expect("Failed to get payload");
    println!("{}", text);

    match status {
      StatusCode::OK => (),
      StatusCode::TOO_MANY_REQUESTS => return Err(UnauthorizedResponse { success: false, error: "Rate Limited".to_string() }),
      StatusCode::FORBIDDEN => return Err(UnauthorizedResponse { success: false, error: "Forbidden Access".to_string() }),
      _ => return Err(UnauthorizedResponse { success: false, error: status.to_string() })
    }

    let inventory = match serde_json::from_str(&text) {
      Ok(inv) => inv,
      Err(e) => {
        println!("{}",&text);
        panic!("{}",e)
      }
    };
    println!("{:?}", inventory);
    Ok(())
  }

}

impl TradeOfferData {
  fn new() -> TradeOfferData {
    TradeOfferData { newversion: true, version: 4, me: OfferData::new(), them: OfferData::new() }
  }
}

impl OfferData {
  fn new() -> OfferData {
    OfferData { assets: Vec::new(), currency: Vec::new(), ready: false }
  }
}

impl OfferAsset {
  pub fn new(appid: String, contextid: String, amount: String, assetid: String) -> OfferAsset {
    OfferAsset { appid, contextid, amount, assetid }
  }
}

pub fn convert_parterid_to_steamid(partner_id: &String) -> u64 {
  let id = partner_id.parse::<u64>().unwrap();
  return id + 76561197960265728; // id + constant = Steamid64
}