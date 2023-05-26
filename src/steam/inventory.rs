use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::{Client, StatusCode};
use super::Trade::OfferAsset;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Inventory {
  pub assets: Vec<Asset>,
  pub descriptions: Vec<AssetDescription>,
  pub total_inventory_count: i64,
  pub success: i32,
  pub rwgrsn: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Asset {
  pub appid: i64,
  pub contextid: String,
  pub assetid: String,
  pub classid: String,
  pub instanceid: String,
  pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AssetDescription {
  pub appid: i64,
  pub classid: String,
  pub instanceid: String,
  pub currency: i64,
  pub background_color: String,
  pub icon_url: String,
  pub icon_url_large: Option<String>,
  pub descriptions: Vec<Description>,
  pub tradable: i64,
  pub actions: Option<Vec<Action>>,
  pub name: String,
  pub name_color: Option<String>,
  #[serde(rename = "type")]
  pub _type: String,
  pub market_name: String,
  pub market_hash_name: String,
  pub market_actions: Option<Vec<Action>>,
  pub commodity: i64,
  pub market_tradable_restriction: i64,
  pub market_marketable_restriction: Option<i64>,
  pub marketable: i64,
  pub tags: Vec<Tag>,
  pub market_buy_country_restriction: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Description {
  #[serde(rename = "type")]
  pub _type: Option<String>,
  pub value: String,
  pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Action {
  pub link: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tag {
  pub category: String,
  pub internal_name: String,
  pub localized_category_name: String,
  pub localized_tag_name: String,
  pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UnauthorizedResponse {
  pub success: bool,
  pub error: String
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

impl Inventory {
  pub async fn new(steam_id: String, game_id: String, context_id: String) -> Result<Inventory, UnauthorizedResponse> {
    
    let url = format!("https://steamcommunity.com/inventory/{}/{}/{}?l=english", steam_id, game_id, context_id);

    let client = Client::new();
    let res = client.get(url)
      .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36")
      .header("Accept", "application/json")
      .send().await.expect("Failed to get response");

    match res.status() {
      StatusCode::OK => (),
      StatusCode::TOO_MANY_REQUESTS => return Err(UnauthorizedResponse { success: false, error: "Rate Limited".to_string() }),
      StatusCode::FORBIDDEN => return Err(UnauthorizedResponse { success: false, error: "Forbidden Access".to_string() }),
      _ => return Err(UnauthorizedResponse { success: false, error: res.status().to_string() })
    }
    
    let text = res.text().await.expect("Failed to get payload");
    
    let inventory = match serde_json::from_str::<Inventory>(&text) {
      Ok(inv) => inv,
      Err(e) => {
        println!("{}",&text);
        panic!("{}",e)
      }
    };

    Ok(inventory)
  }

  pub fn get_trade_items(&self, items: Vec<AssetDescription>) -> Vec<OfferAsset>{
    let mut assets: Vec<OfferAsset> = Vec::new();
    let mut seen: HashMap<&String, bool> = HashMap::new();
    for item in items {
      for asset in &self.assets {
        let has_seen = seen.get(&asset.assetid).unwrap_or(&false);
        if !has_seen && item.classid == asset.classid && item.instanceid == asset.instanceid {
          assets.push(OfferAsset::new("730".to_string(), asset.contextid.to_owned(), "1".to_string(), asset.assetid.to_owned()));
          seen.insert(&asset.assetid, true);
        }
      }
    }

    assets
  }

  pub fn search_item_name(&self, item_name: String) -> Option<AssetDescription> {
    for description in &self.descriptions {
      if description.market_name.contains(item_name.as_str()) {
        return Some(description.to_owned());
      }
    }
    
    None
  }

  pub fn get_all_rarity(&self, quality: ItemRarity) -> Vec<AssetDescription> {
    let mut results: Vec<AssetDescription> = Vec::new();

    for description in &self.descriptions {
      for tag in &description.tags {
        if tag.category == "Quality" {
          match quality {
            ItemRarity::BaseGrade => if tag.localized_tag_name == "Base Grade" {results.push(description.to_owned())},
            ItemRarity::ConsumerGrade => if tag.localized_tag_name == "Consumer Grade" {results.push(description.to_owned())},
            ItemRarity::IndustrialGrade => if tag.localized_tag_name == "Industrial Grade" {results.push(description.to_owned())},
            ItemRarity::MilspecGrade => if tag.localized_tag_name == "Mil-Spec Grade" {results.push(description.to_owned())},
            ItemRarity::Distinguished => if tag.localized_tag_name == "Distinguished" {results.push(description.to_owned())},
            ItemRarity::HighGrade => if tag.localized_tag_name == "High Grade" {results.push(description.to_owned())},
            ItemRarity::Restricted => if tag.localized_tag_name == "Restricted" {results.push(description.to_owned())},
            ItemRarity::Exceptional => if tag.localized_tag_name == "Exceptional" {results.push(description.to_owned())},
            ItemRarity::Remarkable => if tag.localized_tag_name == "Remarkable" {results.push(description.to_owned())},
            ItemRarity::Classified => if tag.localized_tag_name == "Classified" {results.push(description.to_owned())},
            ItemRarity::Superior => if tag.localized_tag_name == "Superior" {results.push(description.to_owned())},
            ItemRarity::Exotic => if tag.localized_tag_name == "Exotic" {results.push(description.to_owned())},
            ItemRarity::Covert => if tag.localized_tag_name == "Covert" {results.push(description.to_owned())},
            ItemRarity::Extraordinary => if tag.localized_tag_name == "Extraordinary" {results.push(description.to_owned())},
            ItemRarity::Master => if tag.localized_tag_name == "Master" {results.push(description.to_owned())},
            ItemRarity::Contraband => if tag.localized_tag_name == "Contraband" {results.push(description.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_category(&self, category: ItemCategory) -> Vec<AssetDescription> {
    let mut results: Vec<AssetDescription> = Vec::new();

    for description in &self.descriptions {
      for tag in &description.tags {
        if tag.category == "Category" {
          match category {
            ItemCategory::Normal => if tag.localized_tag_name == "Normal" {results.push(description.to_owned())},
            ItemCategory::Souvenir => if tag.localized_tag_name == "Souvenir" {results.push(description.to_owned())},
            ItemCategory::Stattrak => if tag.localized_tag_name == "StatTrak™" {results.push(description.to_owned())},
            ItemCategory::Special => if tag.localized_tag_name == "★" {results.push(description.to_owned())},
            ItemCategory::SpecialStattrak => if tag.localized_tag_name == "★ StatTrak™" {results.push(description.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_exterior(&self, exterior: ItemExterior) -> Vec<AssetDescription> {
    let mut results: Vec<AssetDescription> = Vec::new();

    for description in &self.descriptions {
      for tag in &description.tags {
        if tag.category == "Exterior" {
          match exterior {
            ItemExterior::FactoryNew => if tag.localized_tag_name == "Factory New" {results.push(description.to_owned())},
            ItemExterior::MinimalWear => if tag.localized_tag_name == "Minimal Wear" {results.push(description.to_owned())},
            ItemExterior::FieldTested => if tag.localized_tag_name == "Field-Tested" {results.push(description.to_owned())},
            ItemExterior::WellWorn => if tag.localized_tag_name == "Well-Worn" {results.push(description.to_owned())},
            ItemExterior::BattleScarred => if tag.localized_tag_name == "Battle-Scarred" {results.push(description.to_owned())},
            ItemExterior::NotPainted => if tag.localized_tag_name == "Not Painted" {results.push(description.to_owned())},
          }
        }
      }
    }

    results
  }

  pub fn get_all_type(&self, _type: ItemType) -> Vec<AssetDescription> {
    let mut results: Vec<AssetDescription> = Vec::new();

    for description in &self.descriptions {
      for tag in &description.tags {
        if tag.category == "Type" {
          match _type {
            ItemType::Pistol => if tag.localized_tag_name == "Pistol" {results.push(description.to_owned())},
            ItemType::SMG => if tag.localized_tag_name == "SMG" {results.push(description.to_owned())},
            ItemType::Rifle => if tag.localized_tag_name == "Rifle" {results.push(description.to_owned())},
            ItemType::SniperRifle => if tag.localized_tag_name == "Sniper Rifle" {results.push(description.to_owned())},
            ItemType::Shotgun => if tag.localized_tag_name == "Shotgun" {results.push(description.to_owned())},
            ItemType::Machinegun => if tag.localized_tag_name == "Machinegun" {results.push(description.to_owned())},
            ItemType::Agent => if tag.localized_tag_name == "Agent" {results.push(description.to_owned())},
            ItemType::Container => if tag.localized_tag_name == "Container" {results.push(description.to_owned())},
            ItemType::Knife => if tag.localized_tag_name == "Knife" {results.push(description.to_owned())},
            ItemType::Sticker => if tag.localized_tag_name == "Sticker" {results.push(description.to_owned())},
            ItemType::Gloves => if tag.localized_tag_name == "Gloves" {results.push(description.to_owned())},
            ItemType::Graffiti => if tag.localized_tag_name == "Graffiti" {results.push(description.to_owned())},
            ItemType::MusicKit => if tag.localized_tag_name == "Music Kit" {results.push(description.to_owned())},
            ItemType::Patch => if tag.localized_tag_name == "Patch" {results.push(description.to_owned())},
            ItemType::Collectible => if tag.localized_tag_name == "Collectible" {results.push(description.to_owned())},
            ItemType::Key => if tag.localized_tag_name == "Key" {results.push(description.to_owned())},
            ItemType::Pass => if tag.localized_tag_name == "Pass" {results.push(description.to_owned())},
            ItemType::Gift => if tag.localized_tag_name == "Gift" {results.push(description.to_owned())},
            ItemType::Tag => if tag.localized_tag_name == "Tag" {results.push(description.to_owned())},
            ItemType::Tool => if tag.localized_tag_name == "Tool" {results.push(description.to_owned())},
          }
        }
      }
    }

    results
  }
}