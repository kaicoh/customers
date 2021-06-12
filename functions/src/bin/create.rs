use lambda_runtime::{Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    functions::run(lambda).await
}

async fn lambda(event: Value, _: Context) -> Result<Value, Error> {
    Ok(json!(event))
}
