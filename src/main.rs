//! This example shows how to send messages between the two cores in the RP235x chip.
//!
//! The LED on the RP Pico W board is connected differently. See wifi_blinky.rs.
//! The starting point was the Embassy example, but a lot more things have
//! been added in.


#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Executor;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{Stack, spawn_core1};

// Code to enable I2C communication.
// Idea on I2C is to enable asynchronous: the interrupt handler
// receives the results.
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{Async, Config, I2c, InterruptHandler};
use embassy_rp::peripherals::{I2C1};


use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use embassy_time::Ticker;
use embassy_time::Timer;
use static_cell::StaticCell;

// Enables both defmt macros (INFO! etc) and causes
// halt on panic.
use {defmt_rtt as _, panic_probe as _};

// Stack for the second core, and the two executors one for
// each core.
static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

// Example of a channel, which can hold just 1 element.  Specified
// as what type of mutex to use, what type of data it broadcasts, and
// the number of entries.
static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();

enum LedState {
    On,
    Off,
}

// This is what ensures the I2C interrupt handler is working for I2C1
bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let led = Output::new(p.PIN_25, Level::Low);

    // Setting up I2C1.  Need to actually verify that it is working however.
    let i2c1 = I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, Config::default());
    
    // Second core: Launch a r
    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(led))));
        },
    );

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task(i2c1))));
}

#[embassy_executor::task]
async fn i2ctask(_i2c: embassy_rp::i2c::I2c<'static, I2C1, Async>) {
    info!("I2C task, have I2c");
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        info!("I2C task!");
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn core0_task(i2c: embassy_rp::i2c::I2c<'static, I2C1, Async>) {
    info!("Hello from core 0");
    let s = Spawner::for_current_executor();
    s.await.spawn(i2ctask(i2c)).unwrap();

    let mut ticker = Ticker::every(Duration::from_secs(1));
    Timer::after_millis(500).await;
    let mut ticker2 = Ticker::every(Duration::from_secs(1));
    loop {
        CHANNEL.send(LedState::On).await;
        ticker.next().await;
        info!("Blink on!");
        CHANNEL.send(LedState::Off).await;
        ticker2.next().await;
        info!("Blink off!");
    }
}

#[embassy_executor::task]
async fn core1_task(mut led: Output<'static>) {
    info!("Hello from core 1");
    loop {
        match CHANNEL.receive().await {
            LedState::On => {
                led.set_high();
                info!("BLINK RECV!");
            }
            LedState::Off => led.set_low(),
        }
    }
}
