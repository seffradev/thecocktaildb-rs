use thecocktaildb_rs::{Client, Cocktails};

#[tokio::main]
async fn main() -> Result<(), thecocktaildb_rs::Error> {
    let client = Client::new("1")?;
    let result = Cocktails::by_name(&client, "margarita").await?;
    println!("{:?}", result);
    Ok(())
}
