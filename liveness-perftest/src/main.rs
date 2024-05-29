use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
  GooseAttack::initialize()?
    .register_scenario(scenario!("LoadtestTransactions").register_transaction(transaction!(loadtest_index)))
    .execute()
    .await?;

  Ok(())
}

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
  let g = user.get("/").await?;

  let validate = &Validate::builder().status(200).text("Hello, World!").build();

  validate_and_load_static_assets(user, g, validate).await?;

  Ok(())
}
