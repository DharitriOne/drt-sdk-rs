// The purpose of this test is to directly showcase how the various
// API traits are being used, without the aid of macros.
// All this code is of course always macro-generated.
//
// Since it is more difficult to debug macros directly,
// it is helpful to keep this test as a reference for macro development
// and maintenance.

#![allow(unused)]

use drt_sc::{
    contract_base::ProxyObjNew,
    types::{BigInt, ManagedAddress},
};
use drt_sc_scenario::api::{SingleTxApi, StaticApi};

use crate::module_1::VersionModule;

mod module_1 {
    drt_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait VersionModule: drt_sc::contract_base::ContractBase + Sized {
        fn version(&self) -> BigInt<Self::Api>;

        fn some_async(&self);

        fn callback(&self);
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: drt_sc::contract_base::ContractBase {}

    impl<C> VersionModule for C
    where
        C: AutoImpl,
    {
        fn version(&self) -> BigInt<Self::Api> {
            BigInt::from(100)
        }

        fn some_async(&self) {
            panic!("wooo")
        }

        fn callback(&self) {}
    }

    impl<A> AutoImpl for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers: VersionModule + drt_sc::contract_base::ContractBase {
        #[inline]
        fn call_version(&self) {
            drt_sc::io::call_value_init::not_payable::<Self::Api>();
            let result = self.version();
            drt_sc::io::finish_multi::<Self::Api, _>(&result)
        }

        fn call_some_async(&self) {
            self.some_async();
            drt_sc::io::finish_multi::<Self::Api, _>(&())
        }

        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    self.callback();
                    return true;
                },
                "version" => {
                    self.call_version();
                    true
                },
                _other => false,
            } {
                return true;
            }
            false
        }
    }

    impl<A> EndpointWrappers for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub struct AbiProvider {}

    impl drt_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = drt_sc::api::uncallable::UncallableApi;

        fn abi() -> drt_sc::abi::ContractAbi {
            drt_sc::abi::ContractAbi::default()
        }
    }

    pub trait ProxyTrait: drt_sc::contract_base::ProxyObjBase + Sized {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn version(
            &mut self,
        ) -> drt_sc::types::Tx<
            drt_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            drt_sc::types::FunctionCall<Self::Api>,
            drt_sc::types::OriginalResultMarker<BigInt<Self::Api>>,
        > {
            drt_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("version")
        }
    }
}

mod sampler_adder_proxy {
    #![allow(dead_code)]
    #![allow(clippy::all)]
    use drt_sc::proxy_imports::*;
    pub struct AdderProxy;
    impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for AdderProxy
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        type TxProxyMethods = AdderProxyMethods<Env, From, To, Gas>;
        fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
            AdderProxyMethods { wrapped_tx: tx }
        }
    }
    pub struct AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
    }
    #[rustfmt::skip]
    impl<Env, From, Gas> AdderProxyMethods<Env, From, (), Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        Gas: TxGas<Env>,
    {
        pub fn init<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            initial_value: Arg0,
        ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_deploy()
                .argument(&initial_value)
                .original_result()
        }
    }
    #[rustfmt::skip]
    impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        pub fn upgrade<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            initial_value: Arg0,
        ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_upgrade()
                .argument(&initial_value)
                .original_result()
        }
    }
    #[rustfmt::skip]
    impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
    where
        Env: TxEnv,
        Env::Api: VMApi,
        From: TxFrom<Env>,
        To: TxTo<Env>,
        Gas: TxGas<Env>,
    {
        pub fn sum(
            self,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
            self.wrapped_tx.payment(NotPayable).raw_call("getSum").original_result()
        }
        /// Add desired amount to the storage variable.
        pub fn add<Arg0: ProxyArg<BigUint<Env::Api>>>(
            self,
            value: Arg0,
        ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
            self.wrapped_tx
                .payment(NotPayable)
                .raw_call("add")
                .argument(&value)
                .original_result()
        }
    }
}

mod sample_adder {
    drt_sc::imports!();

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT TRAIT /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait Adder:
        super::module_1::VersionModule + drt_sc::contract_base::ContractBase + Sized
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init(&self, initial_value: drt_sc::types::BigUint<Self::Api>) {
            self.sum().set(initial_value);
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade(&self, initial_value: drt_sc::types::BigUint<Self::Api>) {
            self.init(initial_value);
        }
        /// Add desired amount to the storage variable.
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn add(&self, value: drt_sc::types::BigUint<Self::Api>) {
            self.sum().update(|sum| *sum += value);
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(&self) -> SingleValueMapper<Self::Api, drt_sc::types::BigUint<Self::Api>>;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// AUTO-IMPLEMENTED METHODS ///////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub trait AutoImpl: drt_sc::contract_base::ContractBase {}

    // impl<C> super::module_1::AutoImpl for C where C: AutoImpl {}

    impl<C> Adder for C
    where
        C: AutoImpl + super::module_1::AutoImpl,
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(&self) -> SingleValueMapper<Self::Api, drt_sc::types::BigUint<Self::Api>> {
            let mut ___key___ = drt_sc::storage::StorageKey::<Self::Api>::new(&b"sum"[..]);
            <SingleValueMapper<
            Self::Api,
            drt_sc::types::BigUint<Self::Api>,
        > as drt_sc::storage::mappers::StorageMapper<Self::Api>>::new(___key___)
        }
    }

    impl<A> AutoImpl for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub trait EndpointWrappers:
        Adder + drt_sc::contract_base::ContractBase + super::module_1::EndpointWrappers
    {
        #[inline]
        fn call_sum(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::io::call_value_init::not_payable::<Self::Api>();
            let () = drt_sc::io::load_endpoint_args::<Self::Api, ()>(());
            let result = self.sum();
            drt_sc::io::finish_multi::<Self::Api, _>(&result);
        }
        #[inline]
        fn call_init(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = drt_sc::io::load_endpoint_args::<
                Self::Api,
                (drt_sc::types::BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.init(initial_value);
        }
        #[inline]
        fn call_upgrade(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::io::call_value_init::not_payable::<Self::Api>();
            let (initial_value, ()) = drt_sc::io::load_endpoint_args::<
                Self::Api,
                (drt_sc::types::BigUint<Self::Api>, ()),
            >(("initial_value", ()));
            self.upgrade(initial_value);
        }
        #[inline]
        fn call_add(&self) {
            <Self::Api as drt_sc::api::VMApi>::init_static();
            drt_sc::io::call_value_init::not_payable::<Self::Api>();
            let (value, ()) = drt_sc::io::load_endpoint_args::<
                Self::Api,
                (drt_sc::types::BigUint<Self::Api>, ()),
            >(("value", ()));
            self.add(value);
        }
        fn call(&self, fn_name: &str) -> bool {
            if match fn_name {
                "callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                },
                "init"
                    if <Self::Api as drt_sc::api::VMApi>::external_view_init_override() =>
                {
                    drt_sc::external_view_contract::external_view_contract_constructor::<
                        Self::Api,
                    >();
                    return true;
                },
                "getSum" => {
                    self.call_sum();
                    true
                },
                "init"
                    if !<Self::Api as drt_sc::api::VMApi>::external_view_init_override() =>
                {
                    self.call_init();
                    true
                },
                "upgrade" => {
                    self.call_upgrade();
                    true
                },
                "add" => {
                    self.call_add();
                    true
                },
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector(
            &self,
            mut ___cb_closure___: drt_sc::types::CallbackClosureForDeser<Self::Api>,
        ) -> drt_sc::types::CallbackSelectorResult<Self::Api> {
            drt_sc::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
        }
        fn callback(&self) {}
    }

    impl<A> EndpointWrappers for drt_sc::contract_base::UniversalContractObj<A> where
        A: drt_sc::api::VMApi
    {
    }

    pub struct AbiProvider {}
    impl drt_sc::contract_base::ContractAbiProvider for AbiProvider {
        type Api = drt_sc::api::uncallable::UncallableApi;
        fn abi() -> drt_sc::abi::ContractAbi {
            let mut contract_abi = drt_sc::abi::ContractAbi::new(
                drt_sc::abi::BuildInfoAbi {
                    contract_crate: drt_sc::abi::ContractCrateBuildAbi {
                        name: "adder",
                        version: "0.0.0",
                        git_version: "",
                    },
                    framework: drt_sc::abi::FrameworkBuildAbi::create(),
                },
                &[
                    "One of the simplest smart contracts possible,",
                    "it holds a single variable in storage, which anyone can increment.",
                ],
                "Adder",
                false,
            );
            let mut endpoint_abi = drt_sc::abi::EndpointAbi::new(
                &[],
                "getSum",
                "sum",
                false,
                false,
                drt_sc::abi::EndpointMutabilityAbi::Readonly,
                drt_sc::abi::EndpointTypeAbi::Endpoint,
                &[],
                &[],
                false,
            );
            endpoint_abi
            .add_output::<
                SingleValueMapper<Self::Api, drt_sc::types::BigUint<Self::Api>>,
            >(&[]);
            contract_abi
            .add_type_descriptions::<
                SingleValueMapper<Self::Api, drt_sc::types::BigUint<Self::Api>>,
            >();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = drt_sc::abi::EndpointAbi::new(
                &[],
                "init",
                "init",
                false,
                false,
                drt_sc::abi::EndpointMutabilityAbi::Mutable,
                drt_sc::abi::EndpointTypeAbi::Init,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<drt_sc::types::BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<drt_sc::types::BigUint<Self::Api>>();
            contract_abi.constructors.push(endpoint_abi);
            let mut endpoint_abi = drt_sc::abi::EndpointAbi::new(
                &[],
                "upgrade",
                "upgrade",
                false,
                false,
                drt_sc::abi::EndpointMutabilityAbi::Mutable,
                drt_sc::abi::EndpointTypeAbi::Upgrade,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<drt_sc::types::BigUint<Self::Api>>("initial_value");
            contract_abi.add_type_descriptions::<drt_sc::types::BigUint<Self::Api>>();
            contract_abi.upgrade_constructors.push(endpoint_abi);
            let mut endpoint_abi = drt_sc::abi::EndpointAbi::new(
                &["Add desired amount to the storage variable."],
                "add",
                "add",
                false,
                false,
                drt_sc::abi::EndpointMutabilityAbi::Mutable,
                drt_sc::abi::EndpointTypeAbi::Endpoint,
                &[],
                &[],
                false,
            );
            endpoint_abi.add_input::<drt_sc::types::BigUint<Self::Api>>("value");
            contract_abi.add_type_descriptions::<drt_sc::types::BigUint<Self::Api>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }

    #[allow(non_snake_case)]
    pub mod __wasm__endpoints__ {
        use super::EndpointWrappers;
        pub fn sum<A>()
        where
            A: drt_sc::api::VMApi,
        {
            super::EndpointWrappers::call_sum(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn init<A>()
        where
            A: drt_sc::api::VMApi,
        {
            super::EndpointWrappers::call_init(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn upgrade<A>()
        where
            A: drt_sc::api::VMApi,
        {
            super::EndpointWrappers::call_upgrade(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn add<A>()
        where
            A: drt_sc::api::VMApi,
        {
            super::EndpointWrappers::call_add(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
        pub fn callBack<A>()
        where
            A: drt_sc::api::VMApi,
        {
            super::EndpointWrappers::callback(
                &drt_sc::contract_base::UniversalContractObj::<A>::new(),
            );
        }
    }
    pub trait ProxyTrait:
        drt_sc::contract_base::ProxyObjBase + super::module_1::ProxyTrait
    {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn sum(
            &mut self,
        ) -> drt_sc::types::Tx<
            drt_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            drt_sc::types::FunctionCall<Self::Api>,
            drt_sc::types::OriginalResultMarker<
                SingleValueMapper<Self::Api, drt_sc::types::BigUint<Self::Api>>,
            >,
        > {
            drt_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("getSum")
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn init<Arg0: drt_sc::types::ProxyArg<drt_sc::types::BigUint<Self::Api>>>(
            &mut self,
            initial_value: Arg0,
        ) -> drt_sc::types::Tx<
            drt_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            drt_sc::types::DeployCall<drt_sc::types::TxScEnv<Self::Api>, ()>,
            drt_sc::types::OriginalResultMarker<()>,
        > {
            drt_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .raw_deploy()
                .argument(&initial_value)
                .original_result()
                .to(self.extract_proxy_to())
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn upgrade<
            Arg0: drt_sc::types::ProxyArg<drt_sc::types::BigUint<Self::Api>>,
        >(
            &mut self,
            initial_value: Arg0,
        ) -> drt_sc::types::Tx<
            drt_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            drt_sc::types::FunctionCall<Self::Api>,
            drt_sc::types::OriginalResultMarker<()>,
        > {
            drt_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("upgrade")
                .argument(&initial_value)
        }
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::type_complexity)]
        fn add<Arg0: drt_sc::types::ProxyArg<drt_sc::types::BigUint<Self::Api>>>(
            &mut self,
            value: Arg0,
        ) -> drt_sc::types::Tx<
            drt_sc::types::TxScEnv<Self::Api>,
            (),
            Self::To,
            (),
            (),
            drt_sc::types::FunctionCall<Self::Api>,
            drt_sc::types::OriginalResultMarker<()>,
        > {
            drt_sc::types::TxBaseWithEnv::new_tx_from_sc()
                .to(self.extract_proxy_to())
                .original_result()
                .raw_call("add")
                .argument(&value)
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT ////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    pub struct ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    //////// CONTRACT OBJECT as CONTRACT BASE ///////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////////////////
    impl<A> drt_sc::contract_base::ContractBase for ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        type Api = A;
    }

    impl<A> super::module_1::AutoImpl for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> AutoImpl for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> super::module_1::EndpointWrappers for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> EndpointWrappers for ContractObj<A> where A: drt_sc::api::VMApi {}

    impl<A> drt_sc::contract_base::CallableContract for ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        fn call(&self, fn_name: &str) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
    }

    pub fn contract_obj<A>() -> ContractObj<A>
    where
        A: drt_sc::api::VMApi,
    {
        ContractObj {
            _phantom: core::marker::PhantomData,
        }
    }
    pub struct ContractBuilder;
    impl drt_sc::contract_base::CallableContractBuilder for self::ContractBuilder {
        fn new_contract_obj<A: drt_sc::api::VMApi + Send + Sync>(
            &self,
        ) -> drt_sc::types::heap::Box<dyn drt_sc::contract_base::CallableContract>
        {
            drt_sc::types::heap::Box::new(ContractObj::<A> {
                _phantom: core::marker::PhantomData,
            })
        }
    }

    pub struct Proxy<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> drt_sc::contract_base::ProxyObjBase for Proxy<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = ();

        fn extract_opt_address(
            &mut self,
        ) -> drt_sc::types::ManagedOption<
            Self::Api,
            drt_sc::types::ManagedAddress<Self::Api>,
        > {
            drt_sc::types::ManagedOption::none()
        }

        fn extract_address(&mut self) -> drt_sc::types::ManagedAddress<Self::Api> {
            drt_sc::api::ErrorApiImpl::signal_error(
                &<A as drt_sc::api::ErrorApi>::error_api_impl(),
                drt_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET.as_bytes(),
            )
        }

        fn extract_proxy_to(&mut self) -> Self::To {}
    }

    impl<A> drt_sc::contract_base::ProxyObjNew for Proxy<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type ProxyTo = ProxyTo<A>;

        fn new_proxy_obj() -> Self {
            Proxy {
                _phantom: core::marker::PhantomData,
            }
        }

        fn contract(
            mut self,
            address: drt_sc::types::ManagedAddress<Self::Api>,
        ) -> Self::ProxyTo {
            ProxyTo {
                address: drt_sc::types::ManagedOption::some(address),
            }
        }
    }

    pub struct ProxyTo<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        pub address:
            drt_sc::types::ManagedOption<A, drt_sc::types::ManagedAddress<A>>,
    }

    impl<A> drt_sc::contract_base::ProxyObjBase for ProxyTo<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type Api = A;
        type To = drt_sc::types::ManagedAddress<A>;

        fn extract_opt_address(
            &mut self,
        ) -> drt_sc::types::ManagedOption<
            Self::Api,
            drt_sc::types::ManagedAddress<Self::Api>,
        > {
            core::mem::replace(
                &mut self.address,
                drt_sc::types::ManagedOption::none(),
            )
        }

        fn extract_address(&mut self) -> drt_sc::types::ManagedAddress<Self::Api> {
            let address = core::mem::replace(
                &mut self.address,
                drt_sc::types::ManagedOption::none(),
            );
            address.unwrap_or_sc_panic(drt_sc::err_msg::RECIPIENT_ADDRESS_NOT_SET)
        }

        fn extract_proxy_to(&mut self) -> Self::To {
            self.extract_address()
        }
    }

    impl<A> super::module_1::ProxyTrait for Proxy<A> where A: drt_sc::api::VMApi {}
    impl<A> super::module_1::ProxyTrait for ProxyTo<A> where A: drt_sc::api::VMApi {}

    impl<A> ProxyTrait for Proxy<A> where A: drt_sc::api::VMApi {}
    impl<A> ProxyTrait for ProxyTo<A> where A: drt_sc::api::VMApi {}

    pub struct CallbackProxyObj<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        _phantom: core::marker::PhantomData<A>,
    }

    impl<A> drt_sc::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
    where
        A: drt_sc::api::VMApi + 'static,
    {
        type Api = A;

        fn new_cb_proxy_obj() -> Self {
            CallbackProxyObj {
                _phantom: core::marker::PhantomData,
            }
        }
    }

    pub trait CallbackProxy: drt_sc::contract_base::CallbackProxyObjBase + Sized {
        fn my_callback(self, caller: &Address) -> drt_sc::types::CallbackClosure<Self::Api> {
            let mut ___callback_call___ =
                drt_sc::types::new_callback_call::<Self::Api>("my_callback");
            ___callback_call___.push_endpoint_arg(caller);
            ___callback_call___
        }
    }
    impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: drt_sc::api::VMApi + 'static {}
}

#[test]
fn contract_without_macros_basic() {
    use sample_adder::{Adder, EndpointWrappers, ProxyTrait};

    let adder = sample_adder::contract_obj::<SingleTxApi>();

    adder.init(drt_sc::types::BigUint::from(5u32));
    assert_eq!(drt_sc::types::BigUint::from(5u32), adder.sum().get());

    adder.add(drt_sc::types::BigUint::from(7u32));
    assert_eq!(
        drt_sc::types::BigUint::from(12u32),
        adder.sum().get()
    );

    assert_eq!(BigInt::from(100), adder.version());

    assert!(!adder.call("invalid_endpoint"));

    assert!(adder.call("getSum"));

    let mut own_proxy =
        sample_adder::Proxy::<StaticApi>::new_proxy_obj().contract(ManagedAddress::zero());
    let _ = own_proxy.sum();

    let _ = drt_sc_meta_lib::abi_json::contract_abi::<sample_adder::AbiProvider>();
}

fn world() -> drt_sc_scenario::ScenarioWorld {
    let mut blockchain = drt_sc_scenario::ScenarioWorld::new();
    blockchain.register_contract(
        "drtsc:../../contracts/examples/adder/output/adder.drtsc.json",
        sample_adder::ContractBuilder,
    );
    blockchain
}

#[test]
fn contract_without_macros_scenario() {
    world().run("../../contracts/examples/adder/scenarios/adder.scen.json");
}
