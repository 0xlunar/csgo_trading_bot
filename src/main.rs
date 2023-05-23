use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let inventory = steam::SteamCSGOInventory::new("76561198047314212".to_string()).await.unwrap();
    
    let item = inventory.search_item_name("Shadow Daggers".to_string());

    let items = match item {
        Some(item) => vec![item],
        None => panic!("Failed to find item")
    };

    let trade_items = inventory.get_trade_items(items);

    println!("{:?}", trade_items);
}


