use std::time::Duration;

use event_analytics::errors::WorkerError;

pub async fn run_with_backoff<F, Fut>(mut f: F) -> Result<(), WorkerError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), WorkerError>>,
{
    let mut delay = Duration::from_millis(100);
    for attempt in 1..=3 {
        let res = f().await;
        match res {
            Ok(v) => return Ok(v),
            Err(e) if attempt == 3 => {
                tracing::error!(error = ?e, "flush failed after 3 attempts");
                return Err(e);
            }
            Err(e) => {
                tracing::warn!(attempt, error = ?e, "flush failed, retrying");
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
        }
    }
    unreachable!()
}
