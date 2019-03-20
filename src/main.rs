use std::fmt;
use std::rc::Rc;

use chrono::Utc;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
// use ed25519_dalek::SecretKey;
// use ed25519_dalek::Signature;
use rand::rngs::OsRng;
// use rand::Rng;

use leb128;
use rust_base58::ToBase58;
use tiny_keccak::keccak256;

pub struct Address {
  prefix: u64,
  data: Rc<String>,
  spend: PublicKey,
  view: PublicKey,
}

pub struct Account {
  address: Address,
  timestamp: u64,
}

pub fn unix_timestamp() -> u64 {
  return Utc::now().timestamp() as u64;
}

impl Address {
  pub fn new(prefix: u64, spend: PublicKey, view: PublicKey) -> Address {
    Address {
      prefix,
      spend,
      view,
      data: Rc::new(Address::to(prefix, spend, view)),
    }
  }
  pub fn get(&self) -> &String {
    &*self.data
  }
  fn to(prefix: u64, spend: PublicKey, view: PublicKey) -> String {
    let mut tag = vec![];
    leb128::write::unsigned(&mut tag, prefix).expect("Fail to write data to vector!");
    let spend_array: Vec<u8> = spend.to_bytes().to_vec();
    let view_array: Vec<u8> = view.to_bytes().to_vec();
    let temp = tag.as_slice();
    let given = [&temp, spend_array.as_slice(), view_array.as_slice()].concat();
    let checksum = &keccak256(given.as_slice())[..4];
    let temp2 = tag.as_slice();
    let pre_base58 = [
      &temp2,
      spend_array.as_slice(),
      view_array.as_slice(),
      checksum,
    ]
    .concat();

    let mut base58 = String::new();
    for chunk in pre_base58.as_slice().chunks(8) {
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

impl fmt::Display for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "spend key: {:x?}, view key: {:x?}",
      self.spend.to_bytes(),
      self.view.to_bytes()
    )
  }
}
impl Account {
  pub fn get_address(&self) -> &String {
    return self.address.get();
  }
  pub fn new(prefix: u64) -> Account {
    let mut spend_rng: OsRng = OsRng::new().unwrap();
    let mut view_rng: OsRng = OsRng::new().unwrap();
    let spend_key_pair: Keypair = Keypair::generate(&mut spend_rng);
    let view_key_pair: Keypair = Keypair::generate(&mut view_rng);
    let address: Address = Address::new(prefix, spend_key_pair.public, view_key_pair.public);
    Account {
      address,
      timestamp: unix_timestamp(),
    }
  }
}

fn main() {}

#[cfg(test)]

mod tests {
  use super::*;
  #[test]
  fn should_get_current_time() {
    let now1: u64 = unix_timestamp();
    assert!(now1 > 10000);
    main();
  }

  #[test]
  fn should_create_account() {
    let prefix = 0x3d;
    let acc: Account = Account::new(prefix);
    let now1: u64 = unix_timestamp();

    assert!(acc.address.prefix == prefix);
    assert!(acc.timestamp - now1 < 10);
    println!("{:?}", acc.get_address());
    println!("{:?}", acc.get_address());
    println!("{}", acc.address);
    println!("{:x?}", acc.address.spend.to_bytes());
    println!("{:x?}", acc.address.spend.to_bytes());
    println!("{:x?}", acc.address.view.to_bytes());
    println!("{:x?}", acc.address.view.to_bytes());
  }
}
