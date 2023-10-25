extern crate alloc;

use alloc::vec;
use alloc::{string::String, vec::Vec};

use crate::std::{error::Result, PtrRef, ArrayRef, ObjectRef};
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
        subtitles_ptr: i32,
        skip_times_ptr: i32,
        headers_ptr: i32
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
        name_ptr: i32,
        name_len: i32,
        format: i32,
        default: bool,
        autoselect: bool
    ) -> i32;

    fn create_episode_server_skip_time(
        start_time: f32,
        end_time: f32,
        skip_type: PlaylistEpisodeServerSkipType
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

impl Default for PlaylistEpisodeSource {
    fn default() -> Self {
        Self { 
            id: "default".into(), 
            display_name: "Default".into(), 
            description: None, 
            servers: vec![] 
        }
    }
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
    pub skip_times: Vec<PlaylistEpisodeServerSkipTime>,
    pub headers: Vec<PlaylistEpisodeServerHeader>
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
    pub name: String,
    pub format: PlaylistEpisodeServerSubtitleFormat,
    pub default: bool,
    pub autoselect: bool,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistEpisodeServerSubtitleFormat {
    VTT,
    ASS,
    SRT
}

pub struct PlaylistEpisodeServerSkipTime {
    /// Start time in seconds
    pub start_time: f32,

    /// End time in seconds
    pub end_time: f32,

    /// Skip type
    pub skip_type: PlaylistEpisodeServerSkipType
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistEpisodeServerSkipType {
    OPENING,
    ENDING,
    RECAP
}

pub struct PlaylistEpisodeServerHeader {
    pub key: String,
    pub value: String
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
                playlist_id: "".into(),
                episode_id: "".into(),
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
                playlist_id: "".into(),
                episode_id: "".into(),
                source_id: "".into(),
                server_id: "".into()
            }    
        }        
    }
}

impl From<PlaylistEpisodeSources> for PtrRef {
    fn from(value: PlaylistEpisodeSources) -> Self {
        let sources = ArrayRef::from(value.0);
        let sources_id = sources.ptr();
        core::mem::forget(sources);
        PtrRef::new(sources_id)
    }
}

impl From<PlaylistEpisodeSource> for PtrRef {
    fn from(value: PlaylistEpisodeSource) -> Self {
        let description = optional_str_ptr(value.description);

        let servers = ArrayRef::from(value.servers);
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

impl From<PlaylistEpisodeServerSkipTime> for PtrRef {
    fn from(value: PlaylistEpisodeServerSkipTime) -> Self {
        PtrRef::new(
            unsafe { 
                create_episode_server_skip_time(
                    value.start_time, 
                    value.end_time, 
                    value.skip_type
                ) 
            }
        )
    }
}

impl From<PlaylistEpisodeServerResponse> for PtrRef {
    fn from(value: PlaylistEpisodeServerResponse) -> Self {
        let links = ArrayRef::from(value.links);
        let links_ptr = links.ptr();
        core::mem::forget(links);

        let subtitles = ArrayRef::from(value.subtitles);
        let subtitles_ptr = subtitles.ptr();
        core::mem::forget(subtitles);

        let skip_times = ArrayRef::from(value.skip_times);
        let skip_times_ptr = skip_times.ptr();
        core::mem::forget(skip_times);

        let mut headers = ObjectRef::new();
        for header in value.headers {
            headers.set(&header.key, header.value.into())
        }
        let headers_ptr = headers.ptr();
        core::mem::forget(headers);

        let response_ptr = unsafe {
            create_episode_server_response(
                links_ptr, 
                subtitles_ptr,
                skip_times_ptr,
                headers_ptr
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
                value.name.as_ptr() as i32,
                value.name.len() as i32,
                value.format as i32,
                value.default,
                value.autoselect
            )
        };
        PtrRef::new(ptr)
    }
}