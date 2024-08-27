use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/drt-wrewa-swap-sc.drtsc.json",
        drt_wrewa_swap_sc::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_rewa_rs() {
    world().run("scenarios/unwrap_rewa.scen.json");
}

#[test]
fn wrap_rewa_rs() {
    world().run("scenarios/wrap_rewa.scen.json");
}
