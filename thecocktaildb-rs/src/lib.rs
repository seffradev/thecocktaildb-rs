use derive_more::Display;
use thiserror::Error;

mod client;
mod cocktails;
mod ingredients;

pub use client::Client;
pub use cocktails::Cocktail;
pub use ingredients::Ingredient;

#[derive(Debug, Display, Error)]
pub enum Error {
    Url(#[from] url::ParseError),
}
