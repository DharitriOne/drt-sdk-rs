use drt_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_partial_contract::<alloc_features::AbiProvider, _>(
        "drtsc:output/alloc-features.drtsc.json",
        alloc_features::ContractBuilder,
        "alloc-features",
    );

    blockchain.register_partial_contract::<alloc_features::AbiProvider, _>(
        "drtsc:output/alloc-mem-fail.drtsc.json",
        alloc_features::ContractBuilder,
        "alloc-mem-fail",
    );

    blockchain.register_partial_contract::<alloc_features::AbiProvider, _>(
        "drtsc:output/alloc-mem-leaking.drtsc.json",
        alloc_features::ContractBuilder,
        "alloc-mem-leaking",
    );

    blockchain
}

#[test]
fn boxed_bytes_zeros_rs() {
    world().run("scenarios/boxed_bytes_zeros.scen.json");
}

#[test]
fn echo_async_result_empty_rs() {
    world().run("scenarios/echo_async_result_empty.scen.json");
}

#[test]
fn echo_big_int_nested_alloc_rs() {
    world().run("scenarios/echo_big_int_nested_alloc.scen.json");
}

#[test]
fn echo_boxed_bytes_rs() {
    world().run("scenarios/echo_boxed_bytes.scen.json");
}

#[test]
fn echo_multi_value_tuples_alloc_rs() {
    world().run("scenarios/echo_multi_value_tuples_alloc.scen.json");
}

#[test]
fn echo_ser_ex_1_rs() {
    world().run("scenarios/echo_ser_ex_1.scen.json");
}

#[test]
fn echo_slice_u_8_rs() {
    world().run("scenarios/echo_slice_u8.scen.json");
}

#[test]
fn echo_str_rs() {
    world().run("scenarios/echo_str.scen.json");
}

#[test]
fn echo_str_box_rs() {
    world().run("scenarios/echo_str_box.scen.json");
}

#[test]
fn echo_string_rs() {
    world().run("scenarios/echo_string.scen.json");
}

#[test]
fn echo_varargs_u_32_alloc_rs() {
    world().run("scenarios/echo_varargs_u32_alloc.scen.json");
}

#[test]
fn echo_vec_u_8_rs() {
    world().run("scenarios/echo_vec_u8.scen.json");
}

#[test]
#[ignore]
fn fail_memory_rs() {
    world().run("scenarios/alloc_mem_fail.scen.json");
}

#[test]
#[ignore]
fn leaking_memory_rs() {
    world().run("scenarios/alloc_mem_leaking.scen.json");
}

#[test]
fn managed_buffer_concat_2_rs() {
    world().run("scenarios/managed_buffer_concat_2.scen.json");
}

#[test]
fn managed_buffer_load_slice_rs() {
    world().run("scenarios/managed_buffer_load_slice.scen.json");
}

#[test]
fn managed_buffer_overwrite_rs() {
    world().run("scenarios/managed_buffer_overwrite.scen.json");
}

#[test]
fn managed_buffer_set_slice_rs() {
    world().run("scenarios/managed_buffer_set_slice.scen.json");
}

#[test]
fn only_owner_legacy_rs() {
    world().run("scenarios/only_owner_legacy.scen.json");
}

#[test]
fn sc_result_rs() {
    world().run("scenarios/sc_result.scen.json");
}

#[test]
fn storage_address_rs() {
    world().run("scenarios/storage_address.scen.json");
}

#[test]
fn storage_opt_address_rs() {
    world().run("scenarios/storage_opt_address.scen.json");
}

#[test]
fn storage_vec_u_8_rs() {
    world().run("scenarios/storage_vec_u8.scen.json");
}
