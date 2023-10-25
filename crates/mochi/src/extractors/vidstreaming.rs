pub struct VidStreaming {}

impl VidStreaming {
    pub fn extract(_video_url: &str) -> crate::imports::error::Result<crate::structs::video::PlaylistEpisodeServerResponse> {
        Err(crate::imports::error::MochiError::Unimplemented)
    }
}