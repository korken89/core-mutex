// Some data used by the driver
struct DriverData;

// This is used with mutexes which can be used with reference or making copies of the Mutex
// Many will work but will not be compatible with for example RTFM
struct WrapperWithMutex<T> {
    mtx: T,
    driver: GenericDriver,
}

impl<T> WrapperWithMutex<T>
where
    T: core_mutex::Mutex<Data = DriverData>,
{
    fn new(mtx: T) -> Self {
        Self {
            mtx,
            driver: GenericDriver::new(),
        }
    }

    pub fn do_something(&mut self) {
        self.mtx.lock(|data| {
            // Do something
        });
    }
}

// This driver is generic to the Mutex implementation
struct GenericDriver {
    // Fields
}

impl GenericDriver {
    pub fn new() -> Self {
        GenericDriver {}
    }

    // Here any Mutex can be used, std, RTFM, cortex-m
    pub fn do_something(mtx: &mut impl core_mutex::Mutex<Data = DriverData>) {
        mtx.lock(|data| {
            // Do something
        });
    }
}

fn main() {}
