pub mod abi_json;
pub mod cargo_toml_contents;
pub mod cli;
mod code_report_json;
pub mod contract;
pub mod ei;
pub mod ei_check_json;
pub mod dcdt_attr_file_json;
mod drtsc_file_json;
pub mod print_util;
mod report_info_json;
pub mod tools;
pub mod version;
pub mod version_history;

#[macro_use]
extern crate lazy_static;

pub use cli::{cli_main, multi_contract_config};
