pub struct VidStreaming {}

impl VidStreaming {
    pub fn extract(_video_url: &str) -> mochi_imports::error::Result<mochi_structs::video::PlaylistEpisodeServerResponse> {
        Err(mochi_imports::error::MochiError::Unimplemented)
    }
}