use super::meta::{PlaylistItemsRequest, PlaylistItemsResponse};

pub trait Image {
    fn playlist_images(request: PlaylistItemsRequest) -> PlaylistItemsResponse;
}
