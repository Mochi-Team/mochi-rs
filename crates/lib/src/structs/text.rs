use super::meta::{PlaylistItemsRequest, PlaylistItemsResponse};

pub trait Text {
    fn playlist_texts(request: PlaylistItemsRequest) -> PlaylistItemsResponse;
}
