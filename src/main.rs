use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let account = steam::account::Account::new(dotenv::var("STEAM_USERNAME").unwrap(), dotenv::var("STEAM_PASSWORD").unwrap(), dotenv::var("STEAM_PRIVATE_KEY").unwrap()).await;

    println!("{:?}", account);
    //test_trade().await;

}

async fn test_trade() {
    let partner_id = "76561198136104447".to_string();
    let self_id = "76561198047314212".to_string();
    
    let mut trade_offer = steam::trade::TradeOffer::new("".to_string(), partner_id.to_owned(), "4-7_Dl8d".to_string());
    trade_offer.set_trade_message("Hello World!".to_string());

    let self_inventory = steam::inventory::Inventory::new(self_id).await.unwrap();
    let partner_inventory = steam::inventory::Inventory::new(partner_id).await.unwrap();

    let self_items = match self_inventory.search_item_name("Shadow Daggers".to_string()) {
        Some(item) => vec![item],
        None => panic!("Item not found")
    };

    let partner_items = match partner_inventory.search_item_name("Marble Fade".to_string()) {
        Some(item) => vec![item],
        None => panic!("Item not found")
    };

    let self_trade_items = self_inventory.get_trade_items(self_items);
    let partner_trade_items = partner_inventory.get_trade_items(partner_items);

    for item in self_trade_items {
        trade_offer.add_self_item(item);
    }
    for item in partner_trade_items {
        trade_offer.add_partner_item(item);
    }

    println!("{:?}", trade_offer);
}


