// Code generated by the drt-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use drt_sc::proxy_imports::*;

pub struct VaultProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for VaultProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = VaultProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        VaultProxyMethods { wrapped_tx: tx }
    }
}

pub struct VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, To, Gas> VaultProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        opt_arg_to_echo: Arg0,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, MultiValue2<&'static str, OptionalValue<ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .argument(&opt_arg_to_echo)
            .original_result()
    }
}
