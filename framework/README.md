# Dharitri Smart Contract Framework

The crates in this folder form the Dharitri smart contract framework.

They are as follows:
    - `drt-sc` - the base crate for smart contract libraries, it is the only dependency the smart contract code sees.
    - `drt-sc-derive` - procedural macros for friendlier SC code
    - `drt-sc-meta` - smart contract meta-programming: build system and other tools
    - `drt-sc-scenario` - the main testing tool, contracts are tested by via scenarios
    - `drt-sc-snippets` - base crate for tools that interact with the blockchain
    - `drt-sc-wasm-adapter` - the API that connects contracts to the WASM backend
