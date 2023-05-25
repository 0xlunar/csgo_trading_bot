use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    test_trade().await;

}

async fn test_trade() {
    let mut trade_offer = steam::Trade::TradeOffer::new("https://steamcommunity.com/tradeoffer/new/?partner=87048484&token=gn-X8Nub".to_string());
    trade_offer.set_trade_message("Hello World!".to_string());

    let partner_inventory = steam::Inventory::Inventory::new(trade_offer.partner.to_string()).await.unwrap();

    let partner_items = match partner_inventory.search_item_name("Shadow Daggers".to_string()) {
        Some(item) => vec![item],
        None => panic!("Item not found")
    };

    let partner_trade_items = partner_inventory.get_trade_items(partner_items);

    for item in partner_trade_items {
        trade_offer.add_partner_item(item);
    }

    let account = steam::account::Account::new(dotenv::var("STEAM_USERNAME").unwrap(), dotenv::var("STEAM_PASSWORD").unwrap(), dotenv::var("STEAM_PRIVATE_KEY").unwrap()).await;

    match account {
        Ok(account) => trade_offer.send(&account.cookie).await.expect("Failed to send trade"),
        Err(e) => panic!("{:?}", e)
    }
}


