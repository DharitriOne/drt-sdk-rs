use forwarder_legacy::fwd_nft_legacy::{Color, ForwarderNftModule};
use drt_sc::{contract_base::ContractBase, types::Address};
use drt_sc_scenario::{
    managed_address, managed_biguint, managed_token_id,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, SetStateStep,
    },
    ScenarioWorld, WhiteboxContract,
};

const USER_ADDRESS_EXPR: &str = "address:user";
const FORWARDER_ADDRESS_EXPR: &str = "sc:forwarder_legacy";
const FORWARDER_PATH_EXPR: &str = "drtsc:output/forwarder_legacy.drtsc.json";

const NFT_TOKEN_ID_EXPR: &str = "str:COOL-123456";
const NFT_TOKEN_ID: &[u8] = b"COOL-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(FORWARDER_PATH_EXPR, forwarder_legacy::ContractBuilder);
    blockchain
}

#[test]
fn test_nft_update_attributes_and_send() {
    let mut world = world();

    let forwarder_legacy_code = world.code_expression(FORWARDER_PATH_EXPR);
    let roles = vec![
        "DCTRoleNFTCreate".to_string(),
        "DCTRoleNFTUpdateAttributes".to_string(),
    ];

    world.set_state_step(
        SetStateStep::new()
            .put_account(USER_ADDRESS_EXPR, Account::new().nonce(1))
            .put_account(
                FORWARDER_ADDRESS_EXPR,
                Account::new()
                    .nonce(1)
                    .code(forwarder_legacy_code)
                    .dct_roles(NFT_TOKEN_ID_EXPR, roles),
            ),
    );

    let forwarder_legacy_whitebox =
        WhiteboxContract::new(FORWARDER_ADDRESS_EXPR, forwarder_legacy::contract_obj);

    let original_attributes = Color { r: 0, g: 0, b: 0 };

    world.whitebox_call(
        &forwarder_legacy_whitebox,
        ScCallStep::new().from(USER_ADDRESS_EXPR),
        |sc| {
            sc.nft_create_compact(
                managed_token_id!(NFT_TOKEN_ID),
                managed_biguint!(1),
                original_attributes,
            );

            sc.send().direct_dct(
                &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                &managed_token_id!(NFT_TOKEN_ID),
                1,
                &managed_biguint!(1),
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().dct_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            1,
            "1",
            Some(original_attributes),
        ),
    ));

    let new_attributes = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    world.whitebox_call(
        &forwarder_legacy_whitebox,
        ScCallStep::new()
            .from(USER_ADDRESS_EXPR)
            .dct_transfer(NFT_TOKEN_ID, 1, "1"),
        |sc| {
            sc.nft_update_attributes(managed_token_id!(NFT_TOKEN_ID), 1, new_attributes);

            sc.send().direct_dct(
                &managed_address!(&address_expr_to_address(USER_ADDRESS_EXPR)),
                &managed_token_id!(NFT_TOKEN_ID),
                1,
                &managed_biguint!(1),
            );
        },
    );

    world.check_state_step(CheckStateStep::new().put_account(
        USER_ADDRESS_EXPR,
        CheckAccount::new().dct_nft_balance_and_attributes(
            NFT_TOKEN_ID_EXPR,
            1,
            "1",
            Some(new_attributes),
        ),
    ));
}

fn address_expr_to_address(address_expr: &str) -> Address {
    AddressValue::from(address_expr).to_address()
}
