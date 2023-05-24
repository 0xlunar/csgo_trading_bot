
use rand;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use rsa::{RsaPublicKey, Pkcs1v15Encrypt};
use base64::{Engine as _, engine::general_purpose};
use dotenv;
use steam_guard;

use num::{BigInt, Num};
use std::str::FromStr;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct LoginResponse { 
  success: bool,
  requires_twofactor: bool,
  login_complete: bool,
  transfer_urls: Vec<String>,
  transfer_parameters: TransferParameters,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct TransferParameters { 
  steamid: String,
  token_secure: String,
  auth: String,
  remember_login: bool,
  webcookie: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Account { 
  pub steam_id: String,
  pub logged_in: bool,
  token_secure: String,
  auth: String,
  webcookie: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct RSAKey {
  success: bool,
  publickey_mod: String,
  publickey_exp: String,
  timestamp: String,
  token_gid: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
struct AccountLoginData {
  username: String,
  password: String,
  twofactorcode: String,
  rsatimestamp: String,
  remember_login: bool,
  emailauth: String,
  emailsteamid: String,
  loginfriendlyname: String,
  captcha_text: String,
  captchagid: String,
}

pub struct SteamGuard {
  secret: String,
}

impl Account {
  pub async fn new(username: String, password: String, totp_secret: String) -> Account {

    let rsa_key = RSAKey::new(&username).await;
    let encrypted_password = rsa_key.encrypt_password(password);

    let two_factor_code = SteamGuard::new(totp_secret).generate_code();

    let login_data = AccountLoginData::new(username, encrypted_password, two_factor_code, rsa_key.timestamp);
    
    println!("{:?}", login_data);

    let client = Client::new();
    let res = client.post("https://steamcommunity.com/login/dologin/")
      .header("Accept", "application/json")
      .form(&login_data)
      .send().await.expect("Failed to get response");

    if !res.status().is_success() {
      panic!("Failed to login to account");
    }
    
    let text = res.text().await.expect("Failed to get payload");

    let login_response = match serde_json::from_str::<LoginResponse>(&text) {
      Ok(response) => response,
      Err(e) => panic!("{}", e)
    };

    Account { steam_id: login_response.transfer_parameters.steamid, logged_in: login_response.login_complete, token_secure: login_response.transfer_parameters.token_secure, auth: login_response.transfer_parameters.auth, webcookie: login_response.transfer_parameters.webcookie }

  }
}

impl RSAKey {
  async fn new(username: &String) -> RSAKey {
    let client = Client::new();
    let res = client.get("https://steamcommunity.com/login/getrsakey/").query(&[("username", username)])
      .header("Accept", "application/json")
      .send().await.expect("Failed to get response");

    if !res.status().is_success() {
      panic!("Failed to fetech RSA Key");
    }

    let text = res.text().await.expect("Failed to get payload");
    let rsa_key = match serde_json::from_str::<RSAKey>(&text) {
      Ok(key) => key,
      Err(e) => panic!("{}", e)
    };

    rsa_key
  }

  pub fn encrypt_password(&self, password: String) -> String {
    let mut rng = rand::thread_rng();

    let pk_mod = BigInt::from_str_radix(&self.publickey_mod, 16).unwrap();
    let pk_exp = BigInt::from_str_radix(&self.publickey_exp, 16).unwrap();

    let public_key = match RsaPublicKey::new(rsa::BigUint::new(pk_mod.to_u32_digits().1), rsa::BigUint::new(pk_exp.to_u32_digits().1)) {
      Ok(key) => key,
      Err(e) => panic!("{}", e)
    };

    let encrypted_password = match public_key.encrypt(&mut rng, Pkcs1v15Encrypt, password.as_bytes()) {
      Ok(enc_pass) => enc_pass,
      Err(e) => panic!("{}", e)
    };

    general_purpose::STANDARD.encode(encrypted_password)
  }
}

impl AccountLoginData {
  pub fn new(username: String, encrypted_password: String, twofactorcode: String, rsatimestamp: String) -> AccountLoginData {
    AccountLoginData { 
      username, 
      password: encrypted_password, 
      twofactorcode, 
      rsatimestamp, 
      remember_login: false, 
      emailauth: "".to_string(), 
      emailsteamid: "".to_string(), 
      loginfriendlyname: "Rust-SteamBot".to_string(), 
      captcha_text: "".to_string(), 
      captchagid: "".to_string() 
    }
  }
}

impl SteamGuard {
  pub fn new(secret: String) -> SteamGuard {
    SteamGuard { secret }
  }

  pub fn generate_code(&self) -> String {
    match steam_guard::from_secret(&self.secret) {
      Ok(code) => code,
      Err(e) => panic!("{}", e)
    }
  }
}