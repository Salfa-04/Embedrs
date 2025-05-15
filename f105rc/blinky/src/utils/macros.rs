//!
//! # Macros
//!

///
/// # init_ticker
///
/// Initialize a Ticker with a given period.
///
/// `init_ticker!()` initializes a Ticker with a given period.
///
/// ## Example
/// ```
/// let mut t = init_ticker!(500); // 500ms
///
/// loop {
///   // Do something
///    t.next().await;
/// }
///
/// ```
///
#[macro_export]
macro_rules! init_ticker {
    ($ms:expr) => {{
        use ::embassy_time::Duration;
        use ::embassy_time::Ticker;

        Ticker::every(Duration::from_millis($ms))
    }};
}
