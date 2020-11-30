pub mod sync {
    pub trait Mutex {
        type Data;

        fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}

pub mod driver {
    pub type Result = core::result::Result<(), ()>;

    pub trait DeviceDriver {
        fn compatible(&self) -> &str;

        fn init(&self) -> Result {
            Ok(())
        }
    }
}
