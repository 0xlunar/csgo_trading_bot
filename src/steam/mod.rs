use rand::{rngs::StdRng, RngCore, SeedableRng};

pub mod Inventory;
pub mod account;
pub mod Trade;

pub fn convert_parterid_to_steamid(partner_id: &String) -> u64 {
  let id = partner_id.parse::<u64>().unwrap();
  return id + 76561197960265728; // id + constant = Steamid64
}

pub fn create_session_id() -> String {
  let seed = [0u8; 32];
  let mut rng: StdRng = SeedableRng::from_seed(seed);
  let mut bytes = [0u8; 12];
  rng.fill_bytes(&mut bytes);
  
  hex::encode(&bytes)
}