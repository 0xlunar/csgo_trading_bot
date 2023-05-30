use std::collections::{HashMap, BTreeSet};
use std::iter::FromIterator;
use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
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
  pub partner: String,
  pub tradeoffermessage: String,
  pub json_tradeoffer: TradeOfferData,
  pub trade_offer_create_params: TradeOfferCreateParams,
  pub trade_url: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TradeOfferSuccess {
  pub tradeofferid: String,
  pub need_mobile_confirmation: Option<bool>,
  pub needs_email_confirmation: Option<bool>,
  pub email_domain: Option<String>
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

    let steam_id: u64 = super::convert_parterid_to_steamid(partnerid);

    TradeOffer { partner: steam_id.to_string(), tradeoffermessage: String::new(), json_tradeoffer: TradeOfferData::new(), trade_offer_create_params: TradeOfferCreateParams { trade_offer_access_token: access_token.to_owned() }, trade_url }
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

  pub async fn send(&mut self, cookie: &String) -> Result<TradeOfferSuccess, UnauthorizedResponse> {
    let client = Client::new();

    let form_data = TradeOfferForm::from(&self);

    let cookie = format!("{}sessionid={};", cookie, &form_data.sessionid);

    let res = client.post("https://steamcommunity.com/tradeoffer/new/send")
      .header("Referer", &self.trade_url)
      .header("Cookie", cookie)
      .form(&form_data)
      .send().await.expect("Failed to send request");

      let status = res.status().to_owned();
      let text = res.text().await.expect("Failed to get payload");
  
      match status {
        StatusCode::OK => (),
        _ => return Err(UnauthorizedResponse { status: status.to_string(), error: text })
      }

    let inventory = match serde_json::from_str::<TradeOfferSuccess>(&text) {
      Ok(inv) => inv,
      Err(e) => {
        println!("{}",&text);
        panic!("{}",e)
      }
    };

    Ok(inventory)
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

impl TradeOfferForm {
  pub fn from(trade_offer: &TradeOffer) -> TradeOfferForm {
    let session_id = super::create_session_id();

    let json_data = serde_json::to_string(&trade_offer.json_tradeoffer).unwrap();
    let token_data = serde_json::to_string(&trade_offer.trade_offer_create_params).unwrap();

    TradeOfferForm {
      serverid: "1".to_string(),
      sessionid: session_id,
      partner: trade_offer.partner.to_string(),
      tradeoffermessage: trade_offer.tradeoffermessage.to_owned(),
      json_tradeoffer: json_data,
      captcha: "".to_string(),
      trade_offer_create_params: token_data,
    }
  }
}