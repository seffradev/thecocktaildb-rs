use derive_more::Display;
use thiserror::Error;

mod alcoholics;
mod categories;
mod client;
mod cocktails;
mod glasses;
mod ingredients;

pub use alcoholics::Alcoholics;
pub use categories::Categories;
pub use client::Client;
pub use cocktails::Cocktails;
pub use glasses::Glasses;
pub use ingredients::Ingredients;

#[derive(Debug, Display, Error)]
pub enum Error {
    Url(#[from] url::ParseError),
    Reqwest(#[from] reqwest::Error),
}
