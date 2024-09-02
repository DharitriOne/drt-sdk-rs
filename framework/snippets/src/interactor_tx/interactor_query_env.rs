use drt_sc_scenario::{
    api::StaticApi,
    drt_sc::types::{ManagedAddress, ManagedBuffer, Tx, TxBaseWithEnv, TxEnv},
    scenario_model::TxExpect,
    ScenarioTxEnv, ScenarioTxEnvData,
};

use crate::Interactor;

impl Interactor {
    pub fn query(&mut self) -> TxBaseWithEnv<InteractorEnvQuery<'_>> {
        let data = self.new_env_data();
        let env = InteractorEnvQuery { world: self, data };
        Tx::new_with_env(env)
    }
}

pub struct InteractorEnvQuery<'w> {
    pub world: &'w mut Interactor,
    pub data: ScenarioTxEnvData,
}

impl<'w> TxEnv for InteractorEnvQuery<'w> {
    type Api = StaticApi;

    type RHExpect = TxExpect;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api> {
        panic!("Explicit sender address expected")
    }

    fn default_gas_annotation(&self) -> ManagedBuffer<Self::Api> {
        self.data.default_gas_annotation()
    }

    fn default_gas_value(&self) -> u64 {
        self.data.default_gas_value()
    }
}

impl<'w> ScenarioTxEnv for InteractorEnvQuery<'w> {
    fn env_data(&self) -> &ScenarioTxEnvData {
        &self.data
    }
}
