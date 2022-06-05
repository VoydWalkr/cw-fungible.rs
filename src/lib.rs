use std::{cmp::Ordering, str::FromStr, fmt::Display};
use cosmwasm_std::{Addr, StdError};
use cw_storage_plus::{PrimaryKey, KeyDeserialize, Key, Prefixer};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub type Result<T> = std::result::Result<T, String>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Fungible {
  Coin(String),
  Token(Addr),
}

impl PartialOrd for Fungible {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other {
      return Some(Ordering::Equal);
    }
    
    match (self, other) {
      (Fungible::Coin(a), Fungible::Coin(b)) => a.partial_cmp(&b),
      (Fungible::Token(a), Fungible::Token(b)) => a.partial_cmp(b),
      (Fungible::Coin(_), Fungible::Token(_)) => Some(Ordering::Greater),
      (Fungible::Token(_), Fungible::Coin(_)) => Some(Ordering::Less),
    }
  }
}

impl Display for Fungible {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Fungible::Coin(coin) => write!(f, "Coin({})", coin),
      Fungible::Token(addr) => write!(f, "Token({})", addr),
    }
  }
}

impl From<Fungible> for String {
  fn from(fungible: Fungible) -> String {
    String::from(&fungible)
  }
}

impl<'a> From<&'a Fungible> for String {
  fn from(fungible: &'a Fungible) -> String {
    match fungible {
      Fungible::Coin(coin) => format!("Coin({})", coin),
      Fungible::Token(token) => format!("Token({})", token.to_string()),
    }
  }
}

impl FromStr for Fungible {
  type Err = String;
  
  fn from_str(s: &str) -> Result<Self> {
    if s.starts_with("Coin(") && s.ends_with(')') {
      let coin = &s[5..s.len() - 1];
      Ok(Fungible::Coin(coin.to_string()))
    }
    else if s.starts_with("Token(") && s.ends_with(')') {
      let token = &s[6..s.len() - 1];
      Ok(Fungible::Token(Addr::unchecked(token)))
    }
    else {
      Err(format!("Invalid fungible: {}", s))
    }
  }
}

impl KeyDeserialize for Fungible {
  type Output = Self;

  fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
    match value[0] {
      0 => Ok(Fungible::Coin(String::from_vec(value[1..].to_vec()).unwrap())),
      1 => Ok(Fungible::Token(Addr::from_vec(value[1..].to_vec()).unwrap())),
      _ => Err(StdError::ParseErr {
        target_type: "Fungible".to_string(),
        msg: "Invalid type byte".to_string(),
      }),
    }
  }
}

impl<'a> PrimaryKey<'a> for Fungible {
  type Prefix = u8;
  type SubPrefix = ();
  type Suffix = Self;
  type SuperSuffix = Self;

  fn key(&self) -> Vec<cw_storage_plus::Key> {
    match self {
      Fungible::Coin(coin) => vec![Key::Ref(coin.as_bytes())],
      Fungible::Token(token) => vec![Key::Ref(token.as_bytes())],
    }
  }
}

impl<'a> Prefixer<'a> for Fungible {
  fn prefix(&self) -> Vec<Key> {
    match self {
      Fungible::Coin(_) => vec![Key::Val8([0u8])],
      Fungible::Token(_) => vec![Key::Val8([1u8])],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cosmwasm_std::testing::MockStorage;
  use cw_storage_plus::Map;
  
  #[test]
  fn test_comparison() {
    let coin1 = Fungible::Coin("abc".to_string());
    let coin2 = Fungible::Coin("def".to_string());
    let token1 = Fungible::Token(Addr::unchecked("token1"));
    let token2 = Fungible::Token(Addr::unchecked("token2"));

    // equality
    assert!(coin1 == coin1);
    assert!(token1 == token1);
    
    // coin-coin
    assert!(coin1 < coin2);
    assert!(coin2 > coin1);
    
    // coin-token, token-coin
    assert!(coin1 > token1);
    assert!(token1 < coin1);
    
    // token-token
    assert!(token1 < token2);
    assert!(token2 > token1);
  }
  
  #[test]
  fn test_storage_primarykey() {
    let mut store = MockStorage::new();
    let map = Map::<Fungible, String>::new("test");
    let coin = Fungible::Coin("uluna".to_string());
    
    map.save(&mut store, coin.clone(), &"abc".to_string()).unwrap();
    assert_eq!(map.load(&mut store, coin.clone()).unwrap(), "abc".to_string());
  }
  
  #[test]
  fn test_storage_tuplekey() {
    let mut store = MockStorage::new();
    let map = Map::<(Fungible, Fungible), String>::new("test");
    let coin = Fungible::Coin("uluna".to_string());
    let token = Fungible::Token(Addr::unchecked("whDAI"));
    
    map.save(&mut store, (coin.clone(), token.clone()), &"abc".to_string()).unwrap();
    assert_eq!(map.load(&mut store, (coin.clone(), token.clone())).unwrap(), "abc".to_string());
  }
  
  #[test]
  fn test_stringify() {
    let coin = Fungible::Coin("uluna".to_string());
    let token = Fungible::Token(Addr::unchecked("whDAI"));
    
    assert_eq!(coin.to_string(), "Coin(uluna)");
    assert_eq!(token.to_string(), "Token(whDAI)");
  }
  
  #[test]
  fn test_parse() {
    let coin_str = "Coin(uluna)";
    let token_str = "Token(whDAI)";
    
    assert_eq!(Fungible::from_str(coin_str).unwrap(), Fungible::Coin("uluna".to_string()));
    assert_eq!(Fungible::from_str(token_str).unwrap(), Fungible::Token(Addr::unchecked("whDAI")));
  }
}
