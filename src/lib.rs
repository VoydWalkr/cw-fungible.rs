use std::cmp::Ordering;
use cosmwasm_std::{Addr};
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

#[cfg(test)]
mod tests {
  use super::*;
  
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
}
