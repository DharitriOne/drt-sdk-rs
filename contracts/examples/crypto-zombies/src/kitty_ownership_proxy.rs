// Code generated by the drt-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use drt_sc::proxy_imports::*;

pub struct KittyOwnershipProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for KittyOwnershipProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = KittyOwnershipProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        KittyOwnershipProxyMethods { wrapped_tx: tx }
    }
}

pub struct KittyOwnershipProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> KittyOwnershipProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
        Arg2: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        birth_fee: Arg0,
        opt_gene_science_contract_address: Arg1,
        opt_kitty_auction_contract_address: Arg2,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&birth_fee)
            .argument(&opt_gene_science_contract_address)
            .argument(&opt_kitty_auction_contract_address)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> KittyOwnershipProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_gene_science_contract_address_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setGeneScienceContractAddress")
            .argument(&address)
            .original_result()
    }

    pub fn set_kitty_auction_contract_address_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setKittyAuctionContractAddress")
            .argument(&address)
            .original_result()
    }

    pub fn claim(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claim")
            .original_result()
    }

    pub fn total_supply(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("totalSupply")
            .original_result()
    }

    pub fn balance_of<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("balanceOf")
            .argument(&address)
            .original_result()
    }

    pub fn owner_of<
        Arg0: ProxyArg<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("ownerOf")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn approve<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        to: Arg0,
        kitty_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("approve")
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        to: Arg0,
        kitty_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transfer")
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn transfer_from<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<u32>,
    >(
        self,
        from: Arg0,
        to: Arg1,
        kitty_id: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transfer_from")
            .argument(&from)
            .argument(&to)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn tokens_of_owner<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, u32>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("tokensOfOwner")
            .argument(&address)
            .original_result()
    }

    pub fn allow_auctioning<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        by: Arg0,
        kitty_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("allowAuctioning")
            .argument(&by)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn approve_siring_and_return_kitty<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<u32>,
    >(
        self,
        approved_address: Arg0,
        kitty_owner: Arg1,
        kitty_id: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("approveSiringAndReturnKitty")
            .argument(&approved_address)
            .argument(&kitty_owner)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn create_gen_zero_kitty(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createGenZeroKitty")
            .original_result()
    }

    pub fn get_kitty_by_id_endpoint<
        Arg0: ProxyArg<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, crate::kitty_obj::Kitty> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getKittyById")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn is_ready_to_breed<
        Arg0: ProxyArg<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isReadyToBreed")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn is_pregnant<
        Arg0: ProxyArg<u32>,
    >(
        self,
        kitty_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isPregnant")
            .argument(&kitty_id)
            .original_result()
    }

    pub fn can_breed_with<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        matron_id: Arg0,
        sire_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("canBreedWith")
            .argument(&matron_id)
            .argument(&sire_id)
            .original_result()
    }

    pub fn approve_siring<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        address: Arg0,
        kitty_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("approveSiring")
            .argument(&address)
            .argument(&kitty_id)
            .original_result()
    }

    pub fn breed_with<
        Arg0: ProxyArg<u32>,
        Arg1: ProxyArg<u32>,
    >(
        self,
        matron_id: Arg0,
        sire_id: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("breedWith")
            .argument(&matron_id)
            .argument(&sire_id)
            .original_result()
    }

    pub fn give_birth<
        Arg0: ProxyArg<u32>,
    >(
        self,
        matron_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("giveBirth")
            .argument(&matron_id)
            .original_result()
    }

    pub fn birth_fee(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("birthFee")
            .original_result()
    }
}
