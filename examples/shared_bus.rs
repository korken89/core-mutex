//! Example implementation for how to make a Mutex act and look like an I2C peripheral, similar use
//! as `shared-bus`
//!
//! Thanks to @japaric for churning out the code

#![no_std]
use core_mutex::Mutex;
use core::cell::RefCell;
use embedded_hal::blocking::i2c;

//
// Use a new-type wrapping a mutex, which implements all traits needed by i2c
// Partial implementation bellow
//
struct MutexI2c<M>(pub M)
where
    M: Mutex;

impl<M> i2c::Write for MutexI2c<M>
where
    M: Mutex,
    M::Data: i2c::Write,
{
    type Error = <M::Data as i2c::Write>::Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.lock(|i2c| i2c.write(addr, bytes))
    }
}

//
// Driver crate (temerature sensor)
//
struct Driver1<I>
where
    I: i2c::Write,
{
    i2c: I,
}

impl<I> Driver1<I>
where
    I: i2c::Write,
{
    fn new(i2c: I) -> Self {
        Driver1 { i2c }
    }

    fn temperature(&mut self) -> Result<(), I::Error> {
        unimplemented!()
    }
}

//
// Driver crate (pressure sensor)
//
struct Driver2<I>
where
    I: i2c::Write,
{
    i2c: I,
}

impl<I> Driver2<I>
where
    I: i2c::Write,
{
    fn new(i2c: I) -> Self {
        Driver2 { i2c }
    }

    fn pressure(&mut self) -> Result<(), I::Error> {
        unimplemented!()
    }
}

//
// HAL crate implementing an I2C as a singleton
//
struct I2c1 {
    _0: (),
}

impl I2c1 {
    fn take_once() -> Option<I2c1> {
        unimplemented!()
    }
}

impl i2c::Write for I2c1 {
    type Error = ();

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// APPLICATIONS BELLOW ///////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

//
// Single thread application usage
//
fn app() {
    // single-thread context: no mutex; just a refcell
    let i2c = I2c1::take_once().unwrap();
    let i2c = RefCell::new(i2c);

    let mut driver1 = Driver1::new(MutexI2c(&i2c));
    let mut driver2 = Driver2::new(MutexI2c(&i2c));

    let t = driver1.temperature();
    let p = driver2.pressure();
}

//
// Concurrent application usage
//
static I2C1: cortex_m::Mutex<RefCell<Option<I2c1>>> = cortex_m::Mutex::new(RefCell::new(None));

fn main() {
    // initialize I2C1

    loop {
        let i2c = MutexI2c(&I2C1);
        let mut driver1 = Driver1::new(i2c);

        let t = driver1.temperature();
    }
}

fn interrupt() {
    let i2c = MutexI2c(&I2C1);
    let mut driver2 = Driver2::new(i2c);

    let p = driver2.pressure();
}

//
// RTFM application usage
//
#[init]
fn init(c: init::Context) -> init::LateResources {
    let i2c1 = I2C1::take_once();

    init::LateResources { i2c1 }
}

#[task(resources = [i2c1], priority = 1)]
fn t1(c: t1::Context) {
    let driver1 = Driver1::new(MutexI2c(c.resources.i2c1));

    let t = driver1.temperature();
}

#[task(resources = [i2c1], priority = 2)]
fn t2(c: t2::Context) {
    let driver2 = Driver2::new(MutexI2c(c.resources.i2c1));

    let p = driver2.pressure();
}

