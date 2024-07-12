use thecocktaildb_rs::{Client, Cocktails};

#[tokio::main]
async fn main() -> Result<(), thecocktaildb_rs::Error> {
    let client = Client::new("1")?;
    let result = Cocktails::random(&client).await?;
    println!("{:?}", result);
    Ok(())
}
