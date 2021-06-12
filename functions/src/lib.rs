use futures::future::Future;
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub async fn run<F, A, B, Fut>(lambda: F) -> Result<(), Error>
where
    F: Fn(A, Context) -> Fut + Send + Sync + 'static,
    A: for<'de> Deserialize<'de> + Send + Sync + 'static,
    B: Serialize + Send + Sync + 'static,
    Fut: Future<Output = Result<B, Error>> + Send + 'static,
{
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let func = handler_fn(lambda);
    lambda_runtime::run(func).await?;

    Ok(())
}
