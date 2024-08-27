use drt_sc::imports::*;

use crate::{constants::*, helpers, storage};

#[drt_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint(payFeeAndFundDCT)]
    #[payable("*")]
    fn pay_fee_and_fund_dct(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all_dct_transfers().clone_value();
        let fee = RewaOrDctTokenPayment::from(payments.get(0));
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        payments.remove(0);

        self.make_fund(0u64.into(), payments, address, valability)
    }
    #[endpoint(payFeeAndFundREWA)]
    #[payable("REWA")]
    fn pay_fee_and_fund_rewa(&self, address: ManagedAddress, valability: u64) {
        let mut fund = self.call_value().rewa_value().clone_value();
        let fee_value = self.fee(&RewaOrDctTokenIdentifier::rewa()).get();
        require!(fund > fee_value, "payment not covering fees");

        fund -= fee_value.clone();
        let fee = RewaOrDctTokenPayment::new(RewaOrDctTokenIdentifier::rewa(), 0, fee_value);
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        self.make_fund(fund, ManagedVec::new(), address, valability);
    }

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        require!(!self.deposit(&address).is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let deposit_mapper = self.deposit(&address).get();
        let depositor = deposit_mapper.depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let deposited_fee_token = deposit_mapper.fees.value;
        let fee_amount = self.fee(&deposited_fee_token.token_identifier).get();
        let rewa_payment = self.call_value().rewa_value().clone_value();
        let dct_payment = self.call_value().all_dct_transfers().clone_value();

        let num_tokens = self.get_num_token_transfers(&rewa_payment, &dct_payment);
        self.check_fees_cover_number_of_tokens(num_tokens, fee_amount, deposited_fee_token.amount);

        self.make_fund(rewa_payment, dct_payment, address, valability);
    }

    #[endpoint(depositFees)]
    #[payable("REWA")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().rewa_or_single_dct();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
