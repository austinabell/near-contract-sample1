use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{Lazy, LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

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
    pub records: LookupMap<AccountId, String>,
    pub unique_values: Lazy<UniqueValues>,
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
            unique_values: Lazy::new(
                StorageKey::UniqueValues,
                UniqueValues::new(StorageKey::UniqueValuesSet),
            ),
        }
    }
    /// Returns true if the message is unique
    pub fn set_status(&mut self, message: String) -> bool {
        let account_id = env::signer_account_id();
        self.records.insert(account_id, message.clone());
        // Without the following call of function `set`,
        // it will cause inconsistency of the `UnorderedSet` inside `UniqueValues`.
        // self.unique_values.set(&unique_values);
        self.unique_values.unique_values.insert(message)
    }
    //
    pub fn get_status(&self, account_id: AccountId) -> Option<&String> {
        self.records.get(&account_id)
    }
    //
    pub fn unique_values_count(&self) -> u32 {
        self.unique_values.unique_values.len()
    }
    //
    pub fn contains_message(&self, message: String) -> bool {
        self.unique_values.unique_values.contains(&message)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".parse().unwrap())
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
            "hello",
            contract.get_status("bob_near".parse().unwrap()).unwrap()
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
        let contract = StatusMessage::new();

        testing_env!(get_context(true));
        assert_eq!(None, contract.get_status("francis.near".parse().unwrap()));

        // Must have this because on drop, StatusMessage will write intermediate values to storage
        testing_env!(get_context(false));
    }
}
