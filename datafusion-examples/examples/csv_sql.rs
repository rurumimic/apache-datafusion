use datafusion::prelude::{CsvReadOptions, SessionContext};

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let ctx = SessionContext::new();

    ctx.register_csv(
        "aggregate_test_100",
        &format!("./data/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM aggregate_test_100 \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;

    df.show().await?;

    Ok(())
}
