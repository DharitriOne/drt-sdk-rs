use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/dct-transfer-with-fee.drtsc.json",
        dct_transfer_with_fee::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_rs() {
    world().run("scenarios/claim.scen.json");
}

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn setup_fees_and_transfer_rs() {
    world().run("scenarios/setup_fees_and_transfer.scen.json");
}
