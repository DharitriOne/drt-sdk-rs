use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "drtsc:output/basic-features.drtsc.json",
        basic_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn managed_decimal_test() {
    world().run("scenarios/managed_decimal.scen.json");
}
