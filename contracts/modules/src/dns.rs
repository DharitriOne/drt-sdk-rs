use crate::dns_proxy;

drt_sc::imports!();

/// Standard smart contract module that deals with registering usernames in a DNS contract.
///
/// Dharitri usernames/herotags need to be requested by the beneficiary.
/// For a contract, this means that they need an endpoint via which to request a username from the DNS.
///
#[drt_sc::module]
pub trait DnsModule {
    #[payable("REWA")]
    #[only_owner]
    #[endpoint(dnsRegister)]
    fn dns_register(&self, dns_address: ManagedAddress, name: ManagedBuffer) {
        let payment = self.call_value().rewa_value().clone_value();
        self.tx()
            .to(&dns_address)
            .typed(dns_proxy::DnsProxy)
            .register(name)
            .rewa(payment)
            .async_call_and_exit();
    }
}
