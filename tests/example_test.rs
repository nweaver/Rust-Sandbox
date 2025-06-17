#![no_std]
#![no_main]

// use defmt::*;
// use {defmt_rtt as _, panic_probe as _};
// use embassy_rp::Peripherals;

fn setup_log() {

}

#[cfg(test)]
#[embedded_test::tests(setup=crate::setup_log())]
mod tests {
    use defmt_rtt as _;
    use embassy_rp::Peripherals;
    use core::sync::atomic::AtomicUsize;
    use core::sync::atomic::Ordering;
    // use defmt::panic as defpanic;
    // use panic_probe as _;




    // An optional init function which is called before every test
    // Asyncness is optional, so is the return value
    #[init]
    async fn init() -> Peripherals {
        return embassy_rp::init(Default::default()); 

        // This is needed to ensure that defmt::timestamp links.
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        defmt::timestamp!("{=usize}", COUNT.fetch_add(1, Ordering::Relaxed));
    }

    // Tests can be async (needs feature `embassy`)
    // Tests can take the state returned by the init function (optional)
    #[test]
    async fn takes_state(_state: Peripherals) {
        // warn!("This is a warning log\n");
        assert!(true)
    }

    // Tests can be conditionally enabled (with a cfg attribute)
    #[test]
    fn log() {
        assert!(true);
        assert_eq!(4, 4);
    }

    // Tests can fail with a custom error message by returning a Result
    // Unfortuantely this doesn't give a clean result...
    #[test]
    fn it_fails_with_err() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        panic!("Deformat panic");   
    }

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}
