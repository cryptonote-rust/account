use std::fmt;
use std::rc::Rc;

use chrono::Utc;

use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
// use ed25519_dalek::SecretKey;
// use ed25519_dalek::Signature;
use rand::rngs::OsRng;
// use rand::Rng;

use cryptonote_base58::{from_base58, to_base58};
use leb128;
use tiny_keccak::keccak256;

pub struct Address {
  prefix: u64,
  data: Rc<String>,
  spend: PublicKey,
  view: PublicKey,
}

pub struct Account {
  spend: Keypair,
  view: Keypair,
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
  fn from(prefix: u64, address: &String) -> Address {
    let addr = address.to_string();
    let bytes = from_base58(addr);
    let (given, checksum) = bytes.split_at(bytes.len() - 4);
    let new_checksum = &keccak256(given)[..4];
    if new_checksum != checksum {
      panic!(
        "Checksum error: expected: {:x?}, got: {:x?}!",
        checksum, new_checksum
      );
    }

    let (mut new_prefix_bytes, keys) = given.split_at(given.len() - 64);
    let new_prefix = leb128::read::unsigned(&mut new_prefix_bytes).expect("Fail to read prefix!");
    if prefix != new_prefix {
      panic!(
        "Prefix not match: expected: {}, got: {}!",
        prefix, new_prefix
      );
    }
    let (spend_bytes, view_bytes) = keys.split_at(32);
    let spend: PublicKey = PublicKey::from_bytes(spend_bytes).unwrap();
    let view: PublicKey = PublicKey::from_bytes(view_bytes).unwrap();
    Address {
      prefix,
      spend,
      view,
      data: Rc::new(address.to_string()),
    }
  }
  fn to(prefix: u64, spend: PublicKey, view: PublicKey) -> String {
    let mut tag = vec![];
    leb128::write::unsigned(&mut tag, prefix).expect("Fail to write prefix!");
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
    let base58 = to_base58(pre_base58);
    base58
  }
}

impl fmt::Display for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\nAdderess: \n  spend key: {:x?},\n  view key: {:x?}",
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
    let spend: Keypair = Keypair::generate(&mut spend_rng);
    let view: Keypair = Keypair::generate(&mut view_rng);
    let address: Address = Address::new(prefix, spend.public, view.public);
    Account {
      address,
      spend,
      view,
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
    println!("{:?}\n", acc.get_address());
    println!("{:?}\n", acc.get_address());
    Address::from(prefix, acc.get_address());
    println!("{}\n", acc.address);
    println!("spend: \n {:x?}\n", acc.address.spend.to_bytes());
    println!("view: \n {:x?}\n", acc.address.view.to_bytes());
  }

  #[test]
  fn should_create_account1() {
    let prefix = 0x3d23;
    let acc: Account = Account::new(prefix);
    let now1: u64 = unix_timestamp();

    assert!(acc.address.prefix == prefix);
    assert!(acc.timestamp - now1 < 10);
    println!("{:?}\n", acc.get_address());
    println!("{:?}\n", acc.get_address());
    Address::from(prefix, acc.get_address());
    println!("{}\n", acc.address);
    println!("spend: \n {:x?}\n", acc.address.spend.to_bytes());
    println!("view: \n {:x?}\n", acc.address.view.to_bytes());
  }
}
