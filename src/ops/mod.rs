use xx_core::{
	coroutines::{block_on, check_interrupt, get_context, Task},
	error::*,
	pointer::*
};

use crate::{driver::Driver, *};

mod io;
mod join;
mod select;
mod spawn;
mod timers;

pub use io::*;
pub use join::*;
pub use select::*;
pub use spawn::*;
pub use timers::*;
pub use xx_core::coroutines::{Join, JoinHandle, Select};

#[asynchronous]
async fn internal_get_runtime_context() -> Ptr<Pulse> {
	unsafe { get_context().await.as_ref() }
		.get_runtime::<Pulse>()
		.ok_or_else(|| {
			Error::simple(
				ErrorKind::Other,
				Some("Cannot use xx-pulse functions with a different runtime")
			)
		})
		.unwrap()
}

#[asynchronous]
async fn internal_get_driver() -> Ptr<Driver> {
	unsafe { internal_get_runtime_context().await.as_ref() }.driver()
}
