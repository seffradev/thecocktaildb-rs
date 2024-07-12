use derive_more::Display;
use thiserror::Error;

mod alcoholics;
mod categories;
mod client;
mod cocktails;
mod glasses;
mod ingredients;

pub use alcoholics::Alcoholic;
pub use categories::Category;
pub use client::Client;
pub use cocktails::Cocktail;
pub use glasses::Glass;
pub use ingredients::Ingredient;

#[derive(Debug, Display, Error)]
pub enum Error {
    Url(#[from] url::ParseError),
    Reqwest(#[from] reqwest::Error),
}
