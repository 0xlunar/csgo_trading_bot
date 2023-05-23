use dotenv;

mod steam;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let inventory = steam::SteamCSGOInventory::new("76561198047314212".to_string()).await.unwrap();
    
    let items = inventory.get_all_category(steam::ItemCategory::Normal);
    println!("{:?}", items);
}


