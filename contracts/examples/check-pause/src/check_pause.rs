#![no_std]

use drt_sc::imports::*;

use drt_sc_modules::pause;

#[drt_sc::contract]
pub trait CheckPauseContract: pause::PauseModule {
    #[init]
    fn init(&self) {}

    #[endpoint(checkPause)]
    fn check_pause(&self) -> bool {
        self.is_paused()
    }
}
