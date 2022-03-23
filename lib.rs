use near_sdk::borsh::{self, BorshSerialize,BorshDeserialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[warn(non_camel_case_types)]
pub struct List {
    owner: AccountId,
    records: LookupMap<AccountId, u128>,
    sum: u128,
}

impl Default for List {
    fn default() -> Self {
        env::panic(b"Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl List {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(),"The contract is already initialized");
        List {
            owner: env::signer_account_id(),
            records: LookupMap::new(b"r".to_vec()),
            sum: 0,
        }
    }
    pub fn deposit(&mut self, amount: u128) {
        let account_id: AccountId = self.owner.parse().unwrap();
        Promise:: new(account_id).transfer(amount);
        self.records.insert(&env::signer_account_id(), &amount);
        self.sum += amount;
    }

    pub fn query(&self, account: AccountId) -> Option<u128> {
        return self.records.get(&account);
    }

    pub fn check_sum(&self) -> u128{
        return self.sum;
    }
    
}