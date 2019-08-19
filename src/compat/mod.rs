//! Compatibility layer between futures 0.1 and `std`.

pub mod backward;
pub mod forward;

use futures::future::FutureResult;

/// Convert a `std::future::Future` yielding `Result` into an 0.1 `Future`.
pub fn into_01<T, Item, Error>(future: T) -> backward::Compat<T>
where
    T: std::future::Future<Output = Result<Item, Error>>,
{
    backward::Compat::new(future)
}
