use std::fmt::Debug;
use std::time::Instant;
use std::future::Future;

use futures::Stream;

/// A future representing the notification that an elapsed duration has occurred.
pub trait Delay: Future<Output = Instant> + Send {}

/// A stream representing notifications at a fixed interval.
pub trait Interval: Stream<Item = Instant> + Debug + Send {}
