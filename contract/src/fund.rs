use crate::Contract;
use crate::ContractExt;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Deposit {
  pub account_id: AccountId, 
  pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
  #[payable] // Public - People can attach money
  pub fn deposit(&mut self) -> U128 {
    // Check if investor is not a beneficiary of the contract
    assert!(self.beneficiary != env::predecessor_account_id(), "Already a Beneficairy!, cannot make deposit.");

    // Get who is calling the method and how much $NEAR they attached
    let investor: AccountId = env::predecessor_account_id();
    
    let fund_amount: Balance = env::attached_deposit();

    let mut funded_so_far = self.deposits.get(&investor).unwrap_or(0);

    // let to_transfer: Balance = if funded_so_far == 0 {
    //   // This is the user's first donation, lets register it, which increases storage
    //   assert!(fund_amount > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);

    //   // Subtract the storage cost to the amount to transfer
    //   fund_amount - STORAGE_COST
    // }else{
    //   fund_amount
    // };

    // self.total_funds += fund_amount;

    // Persist in storage the amount funded so far
    funded_so_far += fund_amount;
    self.deposits.insert(&investor, &funded_so_far);
    
    log!("Thank you {} for the funds {}! You deposited a total of {}", investor.clone(), fund_amount, funded_so_far);
        
    // Return the total amount funded so far
    U128(funded_so_far)
  }

  pub fn claim(&mut self, claim_amount: f64) {  
    assert!(
      self.deadline < env::block_timestamp(),
      "Crowdfunding is not closed"
    );

    let requester = env::predecessor_account_id();
    let total_funds = env::account_balance();

    if requester == self.beneficiary {     
      assert!(
        // total_funds >= self.target as u128 && 
        claim_amount >= self.target as f64,
        "Total Funds did not meet the target requirement"
      );

      let to_transfer: Balance = &total_funds - STORAGE_COST;

      // Send the money to the beneficiary
      Promise::new(self.beneficiary.clone()).transfer(to_transfer);
    } else {
      assert!(
        total_funds < self.target as u128,
        "Funding round is successful. You cannot claim now!"
      );

      let funded_so_far = self.deposits.get(&requester).unwrap_or(0);
      // let mut funded_so_far = self.deposits.get(&investor).unwrap_or(0);

      assert!(funded_so_far > 0, "There is no funds from you. Claim not allowed.");

      let to_transfer: Balance = funded_so_far - STORAGE_COST;      

      Promise::new(requester.clone()).transfer(to_transfer);

      // Clear investor balance
      self.deposits.insert(&requester, &0);
    }    
  }

  // Public - get donation by account ID
  pub fn get_deposit_for_account(&self, account_id: AccountId) -> Deposit {
    Deposit {
      account_id: account_id.clone(),
      total_amount: U128(self.deposits.get(&account_id).unwrap_or(0))
    }
  }

  // Public - get total number of investors
  pub fn number_of_investors(&self) -> u64 {
    self.deposits.len()
  }

  // Public - paginate through all deposits on the contract
  pub fn get_deposits(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Deposit> {
    //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
    let start = u128::from(from_index.unwrap_or(U128(0)));

    //iterate through deposits
    self.deposits.keys()
      //skip to the index we specified in the start variable
      .skip(start as usize) 
      //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
      .take(limit.unwrap_or(50) as usize) 
      .map(|account| self.get_deposit_for_account(account))
      //since we turned map into an iterator, we need to turn it back into a vector to return
      .collect()
  }

  pub fn deposits_total() -> u128 {
    env::account_balance()
  }
}