use std::collections::{HashMap, BTreeSet};
use std::iter::FromIterator;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use dotenv;

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
  pub trade_offer_create_params: TradeOfferCreateParams
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TradeOfferCreateParams {
  pub trade_offer_acces_token: String,
}

impl TradeOffer {
  pub fn new(session_id: String, partner_id: String, trade_token: String) -> TradeOffer {
    TradeOffer { sessionid: session_id, serverid: "1".to_string(), partner: partner_id, tradeoffermessage: String::new(), json_tradeoffer: TradeOfferData::new(), captcha: "".to_string(), trade_offer_create_params: TradeOfferCreateParams { trade_offer_acces_token: trade_token } }
  }

  pub fn set_trade_message(&mut self, message: String) {
    self.tradeoffermessage = message;
  }

  pub fn add_self_item(&mut self, asset: OfferAsset) {
    self.json_tradeoffer.me.assets.push(asset);
  }

  pub fn add_self_items(&mut self, assets: Vec<OfferAsset>) {
    for asset in assets {
      self.json_tradeoffer.me.assets.push(asset);
    }
  }

  pub fn add_partner_item(&mut self, asset: OfferAsset) {
    self.json_tradeoffer.them.assets.push(asset);
  }

  pub fn add_partner_items(&mut self, assets: Vec<OfferAsset>) {
    for asset in assets {
      self.json_tradeoffer.them.assets.push(asset);
    }
  }

  pub fn remove_self_item(&mut self, assetid: String) {
    if let Some(idx) = self.json_tradeoffer.me.assets.iter().position(|x| *x.assetid != assetid)  {
      self.json_tradeoffer.me.assets.swap_remove(idx);
    }
  }

  pub fn remove_self_items(&mut self, assetids: Vec<String>) {
    for assetid in assetids {
      if let Some(idx) = self.json_tradeoffer.me.assets.iter().position(|x| *x.assetid != assetid)  {
        self.json_tradeoffer.me.assets.swap_remove(idx);
      }
    }
    
  }

  pub fn remove_partner_item(&mut self, assetid: String) {
    if let Some(idx) = self.json_tradeoffer.them.assets.iter().position(|x| *x.assetid != assetid)  {
      self.json_tradeoffer.them.assets.swap_remove(idx);
    }
  }

  pub fn remove_partner_items(&mut self, assetids: Vec<String>) {
    for assetid in assetids {
      if let Some(idx) = self.json_tradeoffer.them.assets.iter().position(|x| *x.assetid != assetid)  {
        self.json_tradeoffer.them.assets.swap_remove(idx);
      }
    }

    let to_remove = BTreeSet::from_iter(assetids);

    self.json_tradeoffer.them.assets.retain(|e| );



  }

  pub fn toggle_self_ready(&mut self) {
    self.json_tradeoffer.me.ready = !self.json_tradeoffer.me.ready;
  }

  pub fn toggle_partner_ready(&mut self) {
    self.json_tradeoffer.them.ready = !self.json_tradeoffer.them.ready;
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