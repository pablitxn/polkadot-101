use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
  pub value: i8,
}

#[near_bindgen]
impl Counter {
  pub fn get_value(&self) -> i8 {
    return self.value;
  }

  pub fn increment(&mut self) {
    self.value += 1;
    let log_message = format!("New value is: {}", self.value);

    env::log_str(log_message.as_str());
    after_change_hook();
  }

  pub fn decrement(&mut self) {
    self.value -= 1;
    let log_message = format!("New value is: {}", self.value);
    env::log_str(log_message.as_str());

    after_change_hook();
  }

  pub fn reset(&mut self) {
    self.value = 0;
    let log_message = format!("New value is: {}", self.value);

    env::log_str(log_message.as_str());
    after_change_hook();
  }
}

fn after_change_hook() {
  env::log_str("Change successful");
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::{testing_env, AccountId, VMContext};

  fn get_context(is_view: bool) -> VMContext {
    VMContextBuilder::new()
    .signer_account_id(AccountId::new_unchecked("robert.testnet".to_string()))
    .is_view(is_view)
    .build()
  }

  #[test]
  fn increment() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Counter{ value: 0 };
    contract.increment();
    print!("Value after increment: {}", contract.value);
    assert_eq!(1, contract.get_value());
  }

  #[test]
  fn decrement() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Counter{ value: 0 };
    contract.decrement();
    print!("Value after decrement: {}", contract.value);
    assert_eq!(-1, contract.get_value());
  }

  #[test]
  fn reset() {
    let context = get_context(false);
    testing_env!(context);

    let mut contract = Counter{ value: 0 };
    contract.reset();

    print!("Value after decrement: {}", contract.value);
    assert_eq!(0, contract.get_value());
  }
}
