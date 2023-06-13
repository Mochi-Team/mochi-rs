extern crate alloc;

use alloc::{vec::Vec, string::{String, ToString}};
use mochi_imports::{error::Result, core::{PtrRef, ArrayRef}};
use super::conversion::optional_str_ptr;

use super::meta::{PlaylistItemsRequest, PlaylistItemsResponse};

#[link(wasm_import_module = "structs_video")]
extern "C" {
    fn create_episode_source(
        id_ptr: i32,
        id_len: i32,
        display_name_ptr: i32,
        display_name_len: i32,
        description_ptr: i32,
        description_len: i32,
        servers_ptr: i32
    ) -> i32;

    fn create_episode_server(
        id_ptr: i32,
        id_len: i32,
        display_name_ptr: i32,
        display_name_len: i32,
        description_ptr: i32,
        description_len: i32
    ) -> i32;

    fn create_episode_server_response(
        links_ptr: i32,
        subtitles_ptr: i32
    ) -> i32;

    fn create_episode_server_link(
        url_ptr: i32,
        url_len: i32,
        quality: i32,
        format: i32
    ) -> i32;

    fn create_episode_server_subtitle(
        url_ptr: i32,
        url_len: i32,
        language_ptr: i32,
        language_len: i32,
        format: i32
    ) -> i32;
}

pub trait Video {
    fn playlist_episodes(request: PlaylistItemsRequest) -> Result<PlaylistItemsResponse>;
    fn playlist_episode_sources(request: PlaylistEpisodeSourcesRequest) -> Result<PlaylistEpisodeSources>;
    fn playlist_episode_server(request: PlaylistEpisodeServerRequest) -> Result<PlaylistEpisodeServerResponse>;
}

pub struct PlaylistEpisodeSourcesRequest {
    pub playlist_id: String,
    pub episode_id: String
}

pub struct PlaylistEpisodeSources(pub Vec<PlaylistEpisodeSource>);

pub struct PlaylistEpisodeSource {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub servers: Vec<PlaylistEpisodeServer>
}

pub struct PlaylistEpisodeServer {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>
}

pub struct PlaylistEpisodeServerRequest {
    pub playlist_id: String,
    pub episode_id: String,
    pub source_id: String,
    pub server_id: String
}

pub struct PlaylistEpisodeServerResponse {
    pub links: Vec<PlaylistEpisodeServerLink>,
    pub subtitles: Vec<PlaylistEpisodeServerSubtitle>,
    // TODO: Add skip times and move format types to links
    // pub skip_times: Vec<SkipTime>
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistEpisodeServerFormatType {
    HLS,
    DASH
}

pub struct PlaylistEpisodeServerLink {
    pub url: String,
    pub quality: PlaylistEpisodeServerQualityType,
    pub format_type: PlaylistEpisodeServerFormatType,
}

#[repr(u32)]
pub enum PlaylistEpisodeServerQualityType {
    Auto,
    Q1080p,
    Q720p,
    Q480p,
    Q360p,
    Custom(u32)
}

impl Into<i32> for PlaylistEpisodeServerQualityType {
    fn into(self) -> i32 {
        match self {
            PlaylistEpisodeServerQualityType::Auto => 0,
            PlaylistEpisodeServerQualityType::Q1080p => 1080,
            PlaylistEpisodeServerQualityType::Q720p => 720,
            PlaylistEpisodeServerQualityType::Q480p => 480,
            PlaylistEpisodeServerQualityType::Q360p => 360,
            PlaylistEpisodeServerQualityType::Custom(v) => v as i32,
        }
    }
}

pub struct PlaylistEpisodeServerSubtitle {
    pub url: String,
    pub language: String,
    pub format: PlaylistEpisodeServerSubtitleFormat
}

#[repr(C)]
pub enum PlaylistEpisodeServerSubtitleFormat {
    VTT,
    ASS,
    SRT
}

// Into + From Implementations

impl Into<PlaylistEpisodeSourcesRequest> for PtrRef {
    fn into(self) -> PlaylistEpisodeSourcesRequest {
        if let Ok(obj_ref) = self.as_object() {
            let playlist_id = obj_ref.get("playlistId").as_string()
                .unwrap_or_default();
            let episode_id = obj_ref.get("episodeId").as_string()
                .unwrap_or_default();
            PlaylistEpisodeSourcesRequest {
                playlist_id,
                episode_id
            }
        } else {
            PlaylistEpisodeSourcesRequest {
                playlist_id: "".to_string(),
                episode_id: "".to_string(),
            }    
        }
    }
}

impl Into<PlaylistEpisodeServerRequest> for PtrRef {
    fn into(self) -> PlaylistEpisodeServerRequest {
        if let Ok(obj_ref) = self.as_object() {
            let playlist_id = obj_ref.get("playlistId").as_string().unwrap_or_default();
            let episode_id = obj_ref.get("episodeId").as_string().unwrap_or_default();
            let source_id = obj_ref.get("sourceId").as_string().unwrap_or_default();
            let server_id = obj_ref.get("serverId").as_string().unwrap_or_default();
            PlaylistEpisodeServerRequest {
                playlist_id,
                episode_id,
                source_id,
                server_id
            }
        } else {
            PlaylistEpisodeServerRequest {
                playlist_id: "".to_string(),
                episode_id: "".to_string(),
                source_id: "".to_string(),
                server_id: "".to_string()
            }    
        }        
    }
}

impl From<PlaylistEpisodeSources> for PtrRef {
    fn from(value: PlaylistEpisodeSources) -> Self {
        let mut sources = ArrayRef::new();
        for source in value.0 {
            sources.insert(source.into())
        }
        let sources_id = sources.ptr();
        core::mem::forget(sources);
        PtrRef::new(sources_id)
    }
}

impl From<PlaylistEpisodeSource> for PtrRef {
    fn from(value: PlaylistEpisodeSource) -> Self {
        let description = optional_str_ptr(value.description);

        let mut servers = ArrayRef::new();
        for server in value.servers {
            servers.insert(server.into())
        }
        let servers_ptr = servers.ptr();
        core::mem::forget(servers);

        let source_ptr = unsafe {
            create_episode_source(
                value.id.as_ptr() as i32, 
                value.id.len() as i32, 
                value.display_name.as_ptr() as i32, 
                value.display_name.len() as i32, 
                description.0, 
                description.1, 
                servers_ptr
            )
        };
        PtrRef::new(source_ptr)
    }
}

impl From<PlaylistEpisodeServer> for PtrRef {
    fn from(value: PlaylistEpisodeServer) -> Self {
        let description = optional_str_ptr(value.description);
        let server_ptr = unsafe {
            create_episode_server(
                value.id.as_ptr() as i32,
                value.id.len() as i32,
                value.display_name.as_ptr() as i32,
                value.display_name.len() as i32,
                description.0,
                description.1
            )
        };
        PtrRef::new(server_ptr)
    }
}

impl From<PlaylistEpisodeServerResponse> for PtrRef {
    fn from(value: PlaylistEpisodeServerResponse) -> Self {
        let mut links = ArrayRef::new();
        for link in value.links {
            links.insert(link.into());
        }
        let links_ptr = links.ptr();
        core::mem::forget(links);

        let mut subtitles = ArrayRef::new();
        for subtitle in value.subtitles {
            subtitles.insert(subtitle.into());
        }
        let subtitles_ptr = subtitles.ptr();
        core::mem::forget(subtitles);

        let response_ptr = unsafe {
            create_episode_server_response(
                links_ptr, 
                subtitles_ptr
            )
        };
        PtrRef::new(response_ptr)
    }
}

impl From<PlaylistEpisodeServerLink> for PtrRef {
    fn from(value: PlaylistEpisodeServerLink) -> Self {
        let ptr = unsafe {
            create_episode_server_link(
                value.url.as_ptr() as i32, 
                value.url.len() as i32,
                value.quality.into(),
                value.format_type as i32
            )
        };
        PtrRef::new(ptr)
    }
}

impl From<PlaylistEpisodeServerSubtitle> for PtrRef {
    fn from(value: PlaylistEpisodeServerSubtitle) -> Self {
        let ptr = unsafe {
            create_episode_server_subtitle(
                value.url.as_ptr() as i32,
                value.url.len() as i32,
                value.language.as_ptr() as i32,
                value.language.len() as i32,
                value.format as i32
            )
        };
        PtrRef::new(ptr)
    }
}