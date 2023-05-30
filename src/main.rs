use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    test_trade().await;

}

async fn test_trade() {
    let account = steam::account::Account::new(dotenv::var("STEAM_USERNAME").unwrap(), dotenv::var("STEAM_PASSWORD").unwrap(), dotenv::var("STEAM_PRIVATE_KEY").unwrap()).await;

    let account = match account {
        Ok(account) => account,
        Err(e) => panic!("{:?}", e)
    };

    let mut trade_offer = steam::Trade::TradeOffer::new("https://steamcommunity.com/tradeoffer/new/?partner=87048484&token=gn-X8Nub".to_string());
    trade_offer.set_trade_message("Hello World!".to_string());

    let partner_inventory = steam::Inventory::Inventory::new(trade_offer.partner.to_string(),"730".to_string(), "2".to_string()).await.unwrap();
    let self_inventory = steam::Inventory::Inventory::new(account.steam_id, "753".to_string(), "6".to_string()).await.unwrap();

    let partner_items = match partner_inventory.search_item_name("Shadow Daggers".to_string()) {
        Some(item) => vec![item],
        None => panic!("Partner item not found")
    };

    let self_items = match self_inventory.search_item_name("Sunset city".to_string()) {
        Some(item) => vec![item],
        None => panic!("Self item not found")
    };

    let partner_trade_items = partner_inventory.get_trade_items(partner_items);
    let self_trade_items = self_inventory.get_trade_items(self_items);

    trade_offer.add_partner_items(partner_trade_items);
    trade_offer.add_self_items(self_trade_items);

    println!("trade -> {:?}", trade_offer);

    match trade_offer.send(&account.cookie).await {
        Ok(trade) => println!("{:?}", trade),
        Err(e) => println!("{:?}", e)
    }
}


