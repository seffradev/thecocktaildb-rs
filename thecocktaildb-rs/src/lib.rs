use derive_more::Display;
use thiserror::Error;

mod alcoholics;
mod client;
mod cocktails;
mod ingredients;

pub use alcoholics::Alcoholic;
pub use client::Client;
pub use cocktails::Cocktail;
pub use ingredients::Ingredient;

#[derive(Debug, Display, Error)]
pub enum Error {
    Url(#[from] url::ParseError),
    Reqwest(#[from] reqwest::Error),
}
