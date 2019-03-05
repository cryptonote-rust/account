#![allow(non_snake_case)]
use chrono::Utc;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
// use ed25519_dalek::Signature;
use rand::rngs::OsRng;
// use rand::Rng;

use leb128;
use rust_base58::ToBase58;
use tiny_keccak::keccak256;

pub struct Address {
  spend: PublicKey,
  view: PublicKey,
}

pub struct Keys {
  address: Address,
  spend: SecretKey,
  view: SecretKey,
}

pub struct Account {
  address: String,
  keys: Keys,
  prefix: u64,
  timestamp: u64,
}

pub fn unix_timestamp() -> u64 {
  return Utc::now().timestamp() as u64;
}

impl Address {
  fn new(spend: PublicKey, view: PublicKey) -> Address {
    Address {
      spend: spend,
      view: view,
    }
  }
}

impl Keys {
  fn new(address: Address, spend: SecretKey, view: SecretKey) -> Keys {
    Keys {
      address: address,
      spend: spend,
      view: view,
    }
  }
}

impl Address {
  fn generate(&self, prefix: u64) -> String {
    let mut tag = vec![];
    leb128::write::unsigned(&mut tag, prefix).expect("Fail to write data to vector!");
    let spendArray: Vec<u8> = self.spend.to_bytes().to_vec();
    let viewArray: Vec<u8> = self.view.to_bytes().to_vec();
    let temp = tag.as_slice();
    let given = [&temp, spendArray.as_slice(), viewArray.as_slice()].concat();
    let checksum = &keccak256(given.as_slice())[..4];
    let temp2 = tag.as_slice();
    let preBase58 = [
      &temp2,
      spendArray.as_slice(),
      viewArray.as_slice(),
      checksum,
    ].concat();

    let mut base58 = String::new();
    for chunk in preBase58.as_slice().chunks(8) {
      let mut part = chunk.to_base58();
      let exp_len = match chunk.len() {
        8 => 11,
        6 => 9,
        5 => 7,
        _ => panic!("Invalid chunk length: {}", chunk.len()),
      };
      let missing = exp_len - part.len();
      if missing > 0 {
        part.insert_str(0, &"11111111111"[..missing]);
      }
      base58.push_str(&part);
    }
    base58
  }
}

impl Account {
  fn getAddress(&self) -> String {
    return self.address.clone();
  }
  fn new(prefix: u64) -> Account {
    let mut spendRng: OsRng = OsRng::new().unwrap();
    let mut viewRng: OsRng = OsRng::new().unwrap();
    let spendKeypair: Keypair = Keypair::generate(&mut spendRng);
    let viewKeypair: Keypair = Keypair::generate(&mut viewRng);
    let address: Address = Address::new(spendKeypair.public, viewKeypair.public);
    let addressString = address.generate(prefix);
    let keys: Keys = Keys::new(address, spendKeypair.secret, viewKeypair.secret);

    Account {
      prefix: prefix,
      keys: keys,
      address: addressString,
      timestamp: unix_timestamp(),
    }
  }
}

#[cfg(test)]

mod tests {
  use super::*;
  #[test]
  fn should_get_current_time() {
    let now1: u64 = unix_timestamp();
    assert!(now1 > 10000);
  }

  #[test]
  fn should_create_account() {
    let prefix = 0x3d;
    let acc: Account = Account::new(prefix);
    let now1: u64 = unix_timestamp();

    assert!(acc.prefix == prefix);
    assert!(acc.timestamp - now1 < 10);
    println!("{:x?}", acc.keys.spend);
    println!("{:?}", acc.getAddress());
  }
}
