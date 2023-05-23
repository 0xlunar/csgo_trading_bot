use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let inventory = steam::SteamCSGOInventory::new("76561198047314212".to_string()).await.unwrap();
    
    let item = inventory.search_item_name("Melondrama".to_string());

    match item {
        Some(item) => println!("{}", item.market_name),
        None => println!("Failed to find item")
    }
}


