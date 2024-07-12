use thecocktaildb_rs::{Client, Cocktail};

#[tokio::main]
async fn main() -> Result<(), thecocktaildb_rs::Error> {
    let client = Client::new("1")?;
    let result = Cocktail::by_name(&client, "margarita").await;
    println!("{:?}", result);
    Ok(())
}
