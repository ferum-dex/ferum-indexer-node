use serde::{Deserialize, Serialize};
use aptos_types::account_address::AccountAddress;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct FerumConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<AccountAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_block: Option<u64>,
}
