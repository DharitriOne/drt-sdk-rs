use drt_chain_scenario_format::interpret_trait::IntoRaw;
use drt_sc_scenario::{
    imports::Bech32Address,
    scenario_model::{Account, BytesKey, BytesValue, Scenario, SetStateStep, Step},
};
use drt_sdk::{
    data::{address::Address, dct::DctBalance},
    gateway::GatewayProxy,
};
use std::collections::{BTreeMap, HashMap};

/// Called directly from CLI, from `sc-meta`.
///
/// Retrieves an account data via the API,
/// then formats it as a scenario set state step.
pub async fn print_account_as_scenario_set_state(
    api_string: String,
    address_bech32_string: String,
) {
    let api = GatewayProxy::new(api_string);
    let address = Bech32Address::from_bech32_string(address_bech32_string);
    let set_state = retrieve_account_as_scenario_set_state(&api, &address).await;
    let scenario = build_scenario(set_state);
    println!("{}", scenario.into_raw().to_json_string());
}

fn build_scenario(set_state: SetStateStep) -> Scenario {
    Scenario {
        name: None,
        comment: None,
        check_gas: None,
        steps: vec![Step::SetState(set_state)],
    }
}

pub async fn retrieve_account_as_scenario_set_state(
    api: &GatewayProxy,
    address: &Bech32Address,
) -> SetStateStep {
    let sdk_address = Address::from_bech32_string(address.to_bech32_str()).unwrap();
    let sdk_account = api.get_account(&sdk_address).await.unwrap();

    let account_dct = api
        .get_account_dct_tokens(&sdk_address)
        .await
        .unwrap_or_else(|err| {
            panic!("failed to retrieve DCT tokens for address {address}: {err}")
        });
    let account_dct_roles = api
        .get_account_dct_roles(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve DCT roles for address {address}: {err}"));
    let account_storage = api
        .get_account_storage_keys(&sdk_address)
        .await
        .unwrap_or_else(|err| panic!("failed to retrieve storage for address {address}: {err}"));

    let account_state = set_account(
        sdk_account,
        account_storage,
        account_dct,
        account_dct_roles,
    );

    let set_state_step = SetStateStep::new();
    set_state_step.put_account(address, account_state)
}

fn set_account(
    account: drt_sdk::data::account::Account,
    account_storage: HashMap<String, String>,
    account_dct: HashMap<String, DctBalance>,
    account_dct_roles: HashMap<String, Vec<String>>,
) -> Account {
    let mut account_state = Account::new()
        .nonce(account.nonce)
        .balance(account.balance.as_str())
        .code(account.code);
    account_state.username = Some(format!("str:{}", account.username.as_str()).into());
    account_state.storage = convert_storage(account_storage);

    for (_, dct_balance) in account_dct.iter() {
        let token_id_expr = format!("str:{}", dct_balance.token_identifier);
        account_state =
            account_state.dct_balance(token_id_expr.as_str(), dct_balance.balance.as_str());
    }

    for (token_id, dct_roles) in account_dct_roles {
        let token_id_expr = format!("str:{token_id}");
        account_state = account_state.dct_roles(token_id_expr.as_str(), dct_roles);
    }

    account_state
}

fn convert_storage(account_storage: HashMap<String, String>) -> BTreeMap<BytesKey, BytesValue> {
    account_storage
        .into_iter()
        .filter(|(k, _)| !k.starts_with("454c524f4e44"))
        .map(|(k, v)| (BytesKey::from(k.as_str()), BytesValue::from(v)))
        .collect()
}
