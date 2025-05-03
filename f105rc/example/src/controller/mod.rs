use crate::init_ticker;

#[embassy_executor::task]
pub async fn main(_p: ()) {
    let mut t = init_ticker!(20);

    loop {
        t.next().await
    }
}
