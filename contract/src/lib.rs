use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap};

mod fund;

// Define the default message
// const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // message: String,
    target: u64,
    deadline: u64,
    beneficiary: AccountId,
    deposits: UnorderedMap<AccountId, u128>,// mapping of investor and deposit amount
    // total_funds: u128
}

// pub enum Status {
//     FundingPeriod,
//     Successful,
//     Closed,
// }

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{        
        Self{
            target: 150_000,
            deadline: env::block_timestamp(),
            beneficiary: "crowdfunding.near-l1.testnet".parse().unwrap(),
            deposits: UnorderedMap::new(b"d"),
            // total_funds: 0
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // pub fn fund(&self) {}

    // pub fn claim(&self) {}

    // pub fn status(&self) -> Status {}

    // pub fn project_info(&self) -> String {
    //     "Crowdfunding of Pre-Seed round for CHAKODY LLC"
    // }

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(beneficiary: AccountId, _target: u64, _deadline: u64) -> Self {
        assert!(_target > 0, "Target fund must be more than 0");
        Self {
            target: _target,
            deadline: _deadline,
            beneficiary,
            deposits: UnorderedMap::new(b"d"),
            // total_funds: 0,
        }
    }

    // Public - beneficiary getter
    pub fn get_beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }

    pub fn get_target(&self) -> u64 {
        self.target.clone()
    }

    pub fn get_deadline(&self) -> u64 {
        self.deadline.clone()
    }

    // Public - but only callable by env::current_account_id(). Sets the beneficiary
    #[private]
    pub fn change_beneficiary(&mut self, beneficiary: AccountId) {        
        assert!(
            self.deposits.get(&beneficiary).unwrap_or(0) == 0, 
            "Already an Investor! cannot become beneficiary"
        );

        assert!(
            beneficiary != env::current_account_id(), 
            "Contract cannot become beneficiary"
        );

        self.beneficiary = beneficiary;
    }

    #[private]
    pub fn change_deadline(&mut self, _new_deadline: u64) {
        self.deadline = _new_deadline;
    }

    #[private]
    pub fn change_target(&mut self, _new_target: u64) {
        self.target = _new_target;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::Balance;

  const BENEFICIARY: &str = "beneficiary";
  const NEAR: u128 = 1000000000000000000000000;

  #[test]
  fn initializes() {
      let contract = Contract::init(BENEFICIARY.parse().unwrap());
      assert_eq!(contract.beneficiary, BENEFICIARY.parse().unwrap())
  }

  #[test]
  fn donate() {
      let mut contract = Contract::init(BENEFICIARY.parse().unwrap());

      // Make a donation
      set_context("donor_a", 1*NEAR);
      contract.donate();
      let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

      // Check the donation was recorded correctly
      assert_eq!(first_donation.total_amount.0, 1*NEAR);

      // Make another donation
      set_context("donor_b", 2*NEAR);
      contract.donate();
      let second_donation = contract.get_donation_for_account("donor_b".parse().unwrap());

      // Check the donation was recorded correctly
      assert_eq!(second_donation.total_amount.0, 2*NEAR);

      // User A makes another donation on top of their original
      set_context("donor_a", 1*NEAR);
      contract.donate();
      let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

      // Check the donation was recorded correctly
      assert_eq!(first_donation.total_amount.0, 1*NEAR * 2);

      assert_eq!(contract.number_of_donors(), 2);
  }

  // Auxiliar fn: create a mock context
  fn set_context(predecessor: &str, amount: Balance) {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor.parse().unwrap());
    builder.attached_deposit(amount);

    testing_env!(builder.build());
  }
}
