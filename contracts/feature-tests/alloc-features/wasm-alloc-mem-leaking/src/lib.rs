// Code generated by the drt-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           63
// Async Callback (empty):               1
// Total number of exported functions:  65

#![no_std]

drt_sc_wasm_adapter::allocator!(leaking);
drt_sc_wasm_adapter::panic_handler!();

drt_sc_wasm_adapter::endpoints! {
    alloc_features
    (
        init => init
        echo_h256 => echo_h256
        echo_boxed_array_u8 => echo_boxed_array_u8
        echo_boxed_bytes => echo_boxed_bytes
        echo_slice_u8 => echo_slice_u8
        echo_vec_u8 => echo_vec_u8
        echo_string => echo_string
        echo_str => echo_str
        echo_str_box => echo_str_box
        echo_async_result_empty => echo_async_result_empty
        echo_large_boxed_byte_array => echo_large_boxed_byte_array
        echo_boxed_ser_example_1 => echo_boxed_ser_example_1
        echo_multi_value_tuples => echo_multi_value_tuples
        echo_ser_example_1 => echo_ser_example_1
        echo_vec_of_managed_buffer => echo_vec_of_managed_buffer
        echo_big_int_vec => echo_big_int_vec
        echo_varags_u32 => echo_varags_u32
        echo_varags_big_uint => echo_varags_big_uint
        compute_get_values => compute_get_values
        compute_create_ec => compute_create_ec
        compute_get_ec_length => compute_get_ec_length
        compute_get_priv_key_byte_length => compute_get_priv_key_byte_length
        compute_ec_add => compute_ec_add
        compute_ec_double => compute_ec_double
        compute_is_on_curve_ec => compute_is_on_curve_ec
        compute_scalar_mult => compute_scalar_mult
        compute_scalar_base_mult => compute_scalar_base_mult
        compute_marshal_ec => compute_marshal_ec
        compute_marshal_compressed_ec => compute_marshal_compressed_ec
        compute_unmarshal_ec => compute_unmarshal_ec
        compute_unmarshal_compressed_ec => compute_unmarshal_compressed_ec
        compute_generate_key_ec => compute_generate_key_ec
        only_owner_legacy => only_owner_legacy
        return_sc_error => return_sc_error
        result_ok => result_ok
        result_err_from_bytes => result_err_from_bytes
        result_err_from_string => result_err_from_string
        result_err_from_str => result_err_from_str
        result_echo => result_echo
        result_echo_2 => result_echo_2
        result_echo_3 => result_echo_3
        mbuffer_from_slice => mbuffer_from_slice
        mbuffer_from_boxed_bytes => mbuffer_from_boxed_bytes
        mbuffer_overwrite => mbuffer_overwrite
        mbuffer_append_bytes => mbuffer_append_bytes
        mbuffer_load_slice => mbuffer_load_slice
        mbuffer_set_slice => mbuffer_set_slice
        managed_address_from => managed_address_from
        load_vec_u8 => load_vec_u8
        load_addr => load_addr
        load_opt_addr => load_opt_addr
        is_empty_opt_addr => is_empty_opt_addr
        load_ser_1 => load_ser_1
        store_vec_u8 => store_vec_u8
        store_addr => store_addr
        store_opt_addr => store_opt_addr
        store_ser_1 => store_ser_1
        compare_h256 => compare_h256
        h256_is_zero => h256_is_zero
        boxed_bytes_zeros => boxed_bytes_zeros
        boxed_bytes_concat_2 => boxed_bytes_concat_2
        boxed_bytes_split => boxed_bytes_split
        vec_concat_const => vec_concat_const
        alloc_with_leaking_memory => alloc_with_leaking_memory
    )
}

drt_sc_wasm_adapter::async_callback_empty! {}
