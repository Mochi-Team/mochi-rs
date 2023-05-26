extern crate alloc;

use mochi_imports::error::Result;
use super::meta::{PlaylistItemsRequest, PlaylistItemsResponse};

pub trait Video {
    fn playlist_episodes(request: PlaylistItemsRequest) -> Result<PlaylistItemsResponse>;
}