use thecocktaildb_rs::{Client, Cocktail};

#[tokio::main]
async fn main() -> Result<(), thecocktaildb_rs::Error> {
    let client = Client::new("1")?;
    let result = Cocktail::random(&client).await;
    println!("{:?}", result);
    Ok(())
}
