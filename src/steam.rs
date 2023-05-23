use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use dotenv;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SteamCSGOInventory {
  success: bool,
  #[serde(rename = "rgInventory")]
  rg_inventory: HashMap<String, RgInventoryItem>,
  #[serde(rename = "rgDescriptions")]
  rg_descriptions: HashMap<String, RgItemDescription>,
  more: bool,
  more_start: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RgInventoryItem {
  id: String,
  classid: String,
  instanceid: String,
  amount: String,
  hide_in_china: i32,
  pos: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RgItemDescription {
  pub appid: String,
  pub classid: String,
  pub instanceid: String,
  pub icon_url: String,
  pub icon_url_large: Option<String>,
  pub icon_drag_url: Option<String>,
  pub name: String,
  pub market_hash_name: String,
  pub market_name: String,
  pub name_color: String,
  pub background_color: String,
  #[serde(rename = "type")]
  pub _type: String,
  pub tradable: i32,
  pub marketable: i32,
  pub commodity: i32,
  pub market_tradable_restriction: String,
  pub fraudwarnings: Option<Vec<String>>,
  pub cache_expiration: Option<String>,
  descriptions: Vec<RgItemDescriptionDescription>,
  owner_descriptions: Either<Vec<RgItemDescriptionDescription>, String>,
  actions: Option<Vec<ItemAction>>,
  market_actions: Option<Vec<ItemAction>>,
  tags: Vec<ItemTag>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
enum Either<T, U> {
  Vec(T),
  String(U),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct RgItemDescriptionDescription {
  #[serde(rename = "type")]
  _type: String,
  value: String,
  color: Option<String>,
  app_data: Option<DescriptionAppData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct DescriptionAppData {
  def_index: Option<String>,
  is_itemset_name: Option<i32>,
  limited: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct ItemAction {
  name: String,
  link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct ItemTag {
  internal_name: String,
  name: String,
  category: String,
  category_name: String,
  color: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UnauthorizedResponse {
  success: bool,
  error: String
}

pub enum ItemRarity {
  ConsumerGrade,
  IndustrialGrade,
  MilspecGrade,
  Restricted,
  Classified,
  Covert,
  Contraband,
  BaseGrade,
  Distinguished,
  Exceptional,
  Superior,
  Extraordinary,
  Master,
  HighGrade,
  Remarkable,
  Exotic
}

pub enum ItemCategory {
  Normal,
  Souvenir,
  Stattrak,
  Special,
  SpecialStattrak
}

pub enum ItemExterior {
  FieldTested,
  MinimalWear,
  BattleScarred,
  WellWorn,
  FactoryNew,
  NotPainted
}

pub enum ItemType {
  Pistol,
  SMG,
  Rifle,
  SniperRifle,
  Shotgun,
  Machinegun,
  Agent,
  Container,
  Knife,
  Sticker,
  Gloves,
  Graffiti,
  MusicKit,
  Patch,
  Collectible,
  Key,
  Pass,
  Gift,
  Tag,
  Tool
}

pub struct TradeOffer {
  newversion: bool,
  version: i32,
  me: TradeOfferOffer,
  them: TradeOfferOffer
}

pub struct TradeOfferOffer {
  assets: Vec<TradeOfferAsset>,
  currency: Vec<String>,
  ready: bool
}

pub struct TradeOfferAsset {
  appid: String,
  contextid: String,
  amount: String,
  assetid: String
}

impl SteamCSGOInventory {
  pub async fn new(steam_id: String) -> Result<SteamCSGOInventory, UnauthorizedResponse> {
    dotenv::dotenv().ok();
    let cookie = dotenv::var("STEAM_COOKIE").unwrap();
    let url = format!("https://steamcommunity.com/profiles/{}/inventory/json/730/2/?l=english", steam_id);

    let client = Client::new();
    let res = client.get(url)
      .header("Cookie", cookie)
      .header("Accept", "application/json")
      .send().await.expect("Failed to get response");

    if !res.status().is_success() {
      return Err(UnauthorizedResponse { success: false, error: "Unauthorized, Please signin.".to_string() })
    }
    
    let text = res.text().await.expect("Failed to get payload");

    let inventory = match serde_json::from_str::<SteamCSGOInventory>(&text) {
      Ok(inv) => inv,
      Err(e) => panic!("{}",e)
    };

    Ok(inventory)
  }

  pub fn search_item_name(&self, item_name: String) -> Option<RgItemDescription> {
    for (key, value) in &self.rg_descriptions {
      if value.market_name.contains(item_name.as_str()) {
        println!("{}", key);
        return Some(value.to_owned());
      }
    }
    
    None
  }

  pub fn get_all_rarity(&self, quality: ItemRarity) -> Vec<RgItemDescription> {
    let mut results: Vec<RgItemDescription> = Vec::new();

    for (_, value) in &self.rg_descriptions {
      for tag in &value.tags {
        if tag.category_name == "Quality" {
          match quality {
            ItemRarity::BaseGrade => if tag.name == "Base Grade" {results.push(value.to_owned())},
            ItemRarity::ConsumerGrade => if tag.name == "Consumer Grade" {results.push(value.to_owned())},
            ItemRarity::IndustrialGrade => if tag.name == "Industrial Grade" {results.push(value.to_owned())},
            ItemRarity::MilspecGrade => if tag.name == "Mil-Spec Grade" {results.push(value.to_owned())},
            ItemRarity::Distinguished => if tag.name == "Distinguished" {results.push(value.to_owned())},
            ItemRarity::HighGrade => if tag.name == "High Grade" {results.push(value.to_owned())},
            ItemRarity::Restricted => if tag.name == "Restricted" {results.push(value.to_owned())},
            ItemRarity::Exceptional => if tag.name == "Exceptional" {results.push(value.to_owned())},
            ItemRarity::Remarkable => if tag.name == "Remarkable" {results.push(value.to_owned())},
            ItemRarity::Classified => if tag.name == "Classified" {results.push(value.to_owned())},
            ItemRarity::Superior => if tag.name == "Superior" {results.push(value.to_owned())},
            ItemRarity::Exotic => if tag.name == "Exotic" {results.push(value.to_owned())},
            ItemRarity::Covert => if tag.name == "Covert" {results.push(value.to_owned())},
            ItemRarity::Extraordinary => if tag.name == "Extraordinary" {results.push(value.to_owned())},
            ItemRarity::Master => if tag.name == "Master" {results.push(value.to_owned())},
            ItemRarity::Contraband => if tag.name == "Contraband" {results.push(value.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_category(&self, category: ItemCategory) -> Vec<RgItemDescription> {
    let mut results: Vec<RgItemDescription> = Vec::new();

    for (_, value) in &self.rg_descriptions {
      for tag in &value.tags {
        if tag.category_name == "Category" {
          match category {
            ItemCategory::Normal => if tag.name == "Normal" {results.push(value.to_owned())},
            ItemCategory::Souvenir => if tag.name == "Souvenir" {results.push(value.to_owned())},
            ItemCategory::Stattrak => if tag.name == "StatTrak™" {results.push(value.to_owned())},
            ItemCategory::Special => if tag.name == "★" {results.push(value.to_owned())},
            ItemCategory::SpecialStattrak => if tag.name == "★ StatTrak™" {results.push(value.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_exterior(&self, exterior: ItemExterior) -> Vec<RgItemDescription> {
    let mut results: Vec<RgItemDescription> = Vec::new();

    for (_, value) in &self.rg_descriptions {
      for tag in &value.tags {
        if tag.category_name == "Exterior" {
          match exterior {
            ItemExterior::FactoryNew => if tag.name == "Factory New" {results.push(value.to_owned())},
            ItemExterior::MinimalWear => if tag.name == "Minimal Wear" {results.push(value.to_owned())},
            ItemExterior::FieldTested => if tag.name == "Field-Tested" {results.push(value.to_owned())},
            ItemExterior::WellWorn => if tag.name == "Well-Worn" {results.push(value.to_owned())},
            ItemExterior::BattleScarred => if tag.name == "Battle-Scarred" {results.push(value.to_owned())},
            ItemExterior::NotPainted => if tag.name == "Not Painted" {results.push(value.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_type(&self, _type: ItemType) -> Vec<RgItemDescription> {
    let mut results: Vec<RgItemDescription> = Vec::new();

    for (_, value) in &self.rg_descriptions {
      for tag in &value.tags {
        if tag.category_name == "Type" {
          match _type {
            ItemType::Pistol => if tag.name == "Pistol" {results.push(value.to_owned())},
            ItemType::SMG => if tag.name == "SMG" {results.push(value.to_owned())},
            ItemType::Rifle => if tag.name == "Rifle" {results.push(value.to_owned())},
            ItemType::SniperRifle => if tag.name == "Sniper Rifle" {results.push(value.to_owned())},
            ItemType::Shotgun => if tag.name == "Shotgun" {results.push(value.to_owned())},
            ItemType::Machinegun => if tag.name == "Machinegun" {results.push(value.to_owned())},
            ItemType::Agent => if tag.name == "Agent" {results.push(value.to_owned())},
            ItemType::Container => if tag.name == "Container" {results.push(value.to_owned())},
            ItemType::Knife => if tag.name == "Knife" {results.push(value.to_owned())},
            ItemType::Sticker => if tag.name == "Sticker" {results.push(value.to_owned())},
            ItemType::Gloves => if tag.name == "Gloves" {results.push(value.to_owned())},
            ItemType::Graffiti => if tag.name == "Graffiti" {results.push(value.to_owned())},
            ItemType::MusicKit => if tag.name == "Music Kit" {results.push(value.to_owned())},
            ItemType::Patch => if tag.name == "Patch" {results.push(value.to_owned())},
            ItemType::Collectible => if tag.name == "Collectible" {results.push(value.to_owned())},
            ItemType::Key => if tag.name == "Key" {results.push(value.to_owned())},
            ItemType::Pass => if tag.name == "Pass" {results.push(value.to_owned())},
            ItemType::Gift => if tag.name == "Gift" {results.push(value.to_owned())},
            ItemType::Tag => if tag.name == "Tag" {results.push(value.to_owned())},
            ItemType::Tool => if tag.name == "Tool" {results.push(value.to_owned())},
          }
        }
      }
    }

    results
  }
}

impl TradeOffer {
  pub fn new()
}