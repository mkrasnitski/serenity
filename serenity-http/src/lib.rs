//! The HTTP client which provides functions for sending requests to endpoints in Discord's API.
//!
//! An important function of the REST API is ratelimiting. Requests to endpoints are ratelimited to
//! prevent spam, and once ratelimited Discord will stop performing requests. The library
//! implements protection to pre-emptively ratelimit, to ensure that no wasted requests are made.
//!
//! The client performs two types of requests:
//! - REST API requests, which require authentication to Discord's gateway using a token;
//! - Other requests, which do not require an authorization token.
//!
//! If a request spuriously fails, it will be retried once.

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serenity_utils;

mod client;
mod error;
mod multipart;
mod ratelimiting;
mod request;
mod routing;

use nonmax::NonMaxU16;
pub use reqwest::StatusCode;
use reqwest::{Method, Url};
use serenity_utils::Snowflake;

pub use self::client::*;
pub use self::error::*;
pub use self::multipart::*;
pub use self::ratelimiting::*;
pub use self::request::*;
pub use self::routing::*;

/// An method used for ratelimiting special routes.
///
/// This is needed because [`reqwest`]'s [`Method`] enum does not derive Copy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum LightMethod {
    /// Indicates that a route is for the `DELETE` method only.
    Delete,
    /// Indicates that a route is for the `GET` method only.
    Get,
    /// Indicates that a route is for the `PATCH` method only.
    Patch,
    /// Indicates that a route is for the `POST` method only.
    Post,
    /// Indicates that a route is for the `PUT` method only.
    Put,
}

impl LightMethod {
    #[must_use]
    pub const fn reqwest_method(self) -> Method {
        match self {
            Self::Delete => Method::DELETE,
            Self::Get => Method::GET,
            Self::Patch => Method::PATCH,
            Self::Post => Method::POST,
            Self::Put => Method::PUT,
        }
    }
}

/// Representation of the method of a query to send for the [`Http::get_guilds`],
/// [`Http::get_bans`], and [`Http::get_scheduled_event_users`], functions.
#[non_exhaustive]
pub enum Pagination {
    After(Snowflake),
    Before(Snowflake),
}

/// Representation of the method of a query to send for the [`Http::get_messages`] function.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum MessagePagination {
    After(Snowflake),
    Around(Snowflake),
    Before(Snowflake),
}

/// The maximum number of members the bot can fetch at once
pub const MEMBER_FETCH_LIMIT: NonMaxU16 = match NonMaxU16::new(1000) {
    Some(m) => m,
    None => unreachable!(),
};

/// Discord's official domains. This is used in [`parse_webhook`] and in its corresponding test.
pub const DOMAINS: [&str; 6] = [
    "discord.com",
    "canary.discord.com",
    "ptb.discord.com",
    "discordapp.com",
    "canary.discordapp.com",
    "ptb.discordapp.com",
];

/// Parses the id and token from a webhook url.
///
/// # Examples
///
/// ```rust
/// let url_str = "https://discord.com/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV";
/// let url = url_str.parse().unwrap();
/// let (id, token) = serenity_http::parse_webhook(&url).unwrap();
///
/// assert_eq!(id, 245037420704169985);
/// assert_eq!(token, "ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV");
/// ```
#[must_use]
pub fn parse_webhook(url: &Url) -> Option<(Snowflake, &str)> {
    let (webhook_id, token) = url.path().strip_prefix("/api/webhooks/")?.split_once('/')?;
    if !["http", "https"].contains(&url.scheme())
        || !DOMAINS.contains(&url.domain()?)
        || !(17..=20).contains(&webhook_id.len())
        || !(60..=68).contains(&token.len())
    {
        return None;
    }
    Some((webhook_id.parse().ok()?, token))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_parser() {
        for domain in DOMAINS {
            let url = format!("https://{domain}/api/webhooks/245037420704169985/ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV").parse().unwrap();
            let (id, token) = parse_webhook(&url).unwrap();
            assert_eq!(id, 245037420704169985);
            assert_eq!(
                token,
                "ig5AO-wdVWpCBtUUMxmgsWryqgsW3DChbKYOINftJ4DCrUbnkedoYZD0VOH1QLr-S3sV"
            );
        }
    }
}
