use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/dct-system-sc-mock.drtsc.json",
        dct_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn dct_system_sc_rs() {
    world().run("scenarios/dct_system_sc.scen.json");
}
