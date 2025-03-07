use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/factorial.drtsc.json",
        factorial::ContractBuilder,
    );
    blockchain
}

#[test]
fn factorial_rs() {
    world().run("scenarios/factorial.scen.json");
}
