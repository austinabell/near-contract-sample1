use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, U64};
use near_sdk::{env, near_bindgen, BorshStorageKey, PanicOnDefault};

near_sdk::setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Records,
    UniqueValues,
    UniqueValuesSet,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UniqueValues {
    pub unique_values: UnorderedSet<String>,
    pub other_business_field: u32,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct StatusMessage {
    pub records: LookupMap<String, String>,
    pub unique_values: LazyOption<UniqueValues>,
}

impl UniqueValues {
    pub fn new(storage_key_of_value_set: StorageKey) -> Self {
        Self {
            unique_values: UnorderedSet::new(storage_key_of_value_set),
            other_business_field: 0,
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    ///
    #[init]
    pub fn new() -> Self {
        Self {
            records: LookupMap::new(StorageKey::Records),
            unique_values: LazyOption::new(
                StorageKey::UniqueValues,
                Some(&UniqueValues::new(StorageKey::UniqueValuesSet)),
            ),
        }
    }
    /// Returns true if the message is unique
    pub fn set_status(&mut self, message: String) -> bool {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
        let mut unique_values = self.unique_values.get().unwrap();
        // Without the following call of function `set`,
        // it will cause inconsistency of the `UnorderedSet` inside `UniqueValues`.
        // self.unique_values.set(&unique_values);
        unique_values.unique_values.insert(&message)
    }
    //
    pub fn get_status(&self, account_id: ValidAccountId) -> Option<String> {
        self.records.get(account_id.as_ref())
    }
    //
    pub fn unique_values_count(&self) -> U64 {
        self.unique_values.get().unwrap().unique_values.len().into()
    }
    //
    pub fn contains_message(&self, message: String) -> bool {
        self.unique_values
            .get()
            .unwrap()
            .unique_values
            .contains(&message)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_get_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = StatusMessage::new();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".try_into().unwrap()).unwrap()
        );
    }

    #[test]
    fn set_unique_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = StatusMessage::new();
        // Unique
        assert!(contract.set_status("hello".to_string()));
        // Unique
        assert!(contract.set_status("hello world".to_string()));
        // Not unique. Same as current
        assert!(!contract.set_status("hello world".to_string()));
        // Not unique. Same as older
        assert!(!contract.set_status("hello".to_string()));
        // Unique
        assert!(contract.set_status("hi".to_string()));
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true);
        testing_env!(context);
        let contract = StatusMessage::new();
        assert_eq!(
            None,
            contract.get_status("francis.near".try_into().unwrap())
        );
    }
}
