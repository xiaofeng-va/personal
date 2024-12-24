extern crate std;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder().is_test(true).try_init().unwrap();
    });
}
