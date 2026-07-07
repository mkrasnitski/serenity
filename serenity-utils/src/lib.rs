#[macro_use]
extern crate serde;

mod macros;
pub mod secrets;
mod snowflake;
pub mod timestamp;

use std::borrow::Cow;
use std::path::Path;

use arrayvec::ArrayVec;
use bytes::Bytes;
use small_fixed_array::FixedArray;
use tokio::fs::File;

pub use self::snowflake::*;

#[derive(Clone, Debug)]
pub struct AttachmentData<'a> {
    pub filename: Cow<'static, str>,
    pub kind: AttachmentDataKind<'a>,
}

#[derive(Clone, Debug)]
pub enum AttachmentDataKind<'a> {
    Bytes(Bytes),
    File(&'a File),
    Path(&'a Path),
}

#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct AllowedMentions {
    pub parse: ArrayVec<ParseValue, 3>,
    pub ids: FixedArray<Snowflake>,
    pub replied_user: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParseValue {
    Everyone,
    Users,
    Roles,
}

pub fn spawn_named<F, T>(_name: &str, future: F) -> tokio::task::JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    #[cfg(all(tokio_unstable, feature = "tokio_task_builder"))]
    let handle = tokio::task::Builder::new()
        .name(&*format!("serenity::{}", _name))
        .spawn(future)
        .expect("called outside tokio runtime");
    #[cfg(not(all(tokio_unstable, feature = "tokio_task_builder")))]
    let handle = tokio::spawn(future);
    handle
}
