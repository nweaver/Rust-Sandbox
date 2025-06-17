#![no_std]
#![no_main]

// use defmt::*;
// use {defmt_rtt as _, panic_probe as _};
// use embassy_rp::Peripherals;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    use embassy_rp::Peripherals;


    // An optional init function which is called before every test
    // Asyncness is optional, so is the return value
    #[init]
    async fn init() -> Peripherals {
        return embassy_rp::init(Default::default());
    }

    // Tests can be async (needs feature `embassy`)
    // Tests can take the state returned by the init function (optional)
    #[test]
    async fn takes_state(_state: Peripherals) {
        assert!(true)
    }

    // Tests can be conditionally enabled (with a cfg attribute)
    #[test]
    fn log() {
        assert!(true)
    }

    // Tests can fail with a custom error message by returning a Result
    #[test]
    fn it_fails_with_err() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}


