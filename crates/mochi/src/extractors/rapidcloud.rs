extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use crate::imports::crypto::Crypto;
use crate::imports::error::Result;
use crate::imports::http::RequestMethod;
use crate::imports::http::Request;
use crate::structs::video::PlaylistEpisodeServerLink;
use crate::structs::video::PlaylistEpisodeServerResponse;
use crate::structs::video::PlaylistEpisodeServerSkipTime;
use crate::structs::video::PlaylistEpisodeServerSubtitle;

static FALLBACK_KEY: &'static str = "c1d17096f2ca11b7";
static HOST: &'static str = "https://rapid-cloud.co";

pub struct RapidCloud {}

impl RapidCloud {
    pub fn extract(video_url: &str) -> Result<PlaylistEpisodeServerResponse> {
        // Example: https://rapid-cloud.co/embed-6/fEjafeaEF?k=1
        let id = video_url
            .split("/")
            .last()
            .and_then(|v| v.split("?").nth(0))
            .expect("ID could not be found for rapidcloud link.");

        let url = format!("{}/ajax/embed-6/getSources?id={}", HOST, id);

        let json = Request::new(
            &url, 
            RequestMethod::Get
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .json()?
        .as_object()?;

        let sources_encrypted = json.get("sources")
            .as_string()?;

        let encrypted = json.get("encrypted")
            .as_bool()
            .unwrap_or(false);

        let mut links: Vec<PlaylistEpisodeServerLink> = vec![];

        if encrypted {
            let encrypted_key = Request::new(
                // TODO: Add param at the end so we don't cache request
                "https://raw.githubusercontent.com/enimax-anime/key/e6/key.txt", 
                RequestMethod::Get
            )
            .string()
            .unwrap_or(FALLBACK_KEY.to_string())
            .as_bytes()
            .to_vec();

            let encrypted_sources_bytes = Crypto::base64_parse(&sources_encrypted);
            let encrypted_salt = encrypted_sources_bytes[8..16].to_vec();

            let mut encrypted_key_and_salt = encrypted_key;
            encrypted_key_and_salt.extend(encrypted_salt);

            let mut hashed_key_and_iv: Vec<u8> = vec![];
            let mut digest_buffer: Vec<u8> = Crypto::md5_hash(&encrypted_key_and_salt);
            hashed_key_and_iv.extend(&digest_buffer);

            while hashed_key_and_iv.len() < 48 {
                digest_buffer.extend(&encrypted_key_and_salt);
                digest_buffer = Crypto::md5_hash(&digest_buffer);
                hashed_key_and_iv.extend(&digest_buffer);
            }

            let decrypted = Crypto::aes_decrypt(
                &encrypted_sources_bytes[16..], 
                &hashed_key_and_iv[0..32], 
                &hashed_key_and_iv[32..48]
            );

            let parsed_array = crate::imports::json::parse(decrypted)?
                .as_array()?;

            for item in parsed_array {
                let object = item.as_object()?;
                if let Ok(link) = object.get("file").as_string() {
                    links.push(
                        PlaylistEpisodeServerLink { 
                            url: link, 
                            quality: crate::structs::video::PlaylistEpisodeServerQualityType::Auto,
                            format_type: crate::structs::video::PlaylistEpisodeServerFormatType::HLS
                        }
                    )
                }
            }
        }

        let mut subtitles: Vec<PlaylistEpisodeServerSubtitle> = vec![];
        let tracks = json.get("tracks").as_array().unwrap_or_default();
        for element in tracks {
            let track = element.as_object()?;
            let file = track.get("file")
                .as_string()?;
            let label = track.get("label")
                .as_string()
                .unwrap_or("Unknown".to_string());
            let kind = track.get("kind")
                .as_string()
                .unwrap_or_default();
            let default = track.get("default").as_bool();
            if kind.contains("captions") {
                subtitles.push(
                    PlaylistEpisodeServerSubtitle { 
                        url: file, 
                        name: label, 
                        format: crate::structs::video::PlaylistEpisodeServerSubtitleFormat::VTT,
                        default: default.unwrap_or(false),
                        autoselect: default.unwrap_or(false),
                    }
                )
            }
        }

        let mut skip_times: Vec<PlaylistEpisodeServerSkipTime> = vec![];
        if let Ok(intro) = json.get("intro").as_object() {
            if let (Ok(start), Ok(stop)) = (intro.get("start").as_float(), intro.get("end").as_float()) {
                if stop > 0.0 {
                    skip_times.push(
                        PlaylistEpisodeServerSkipTime { 
                            start_time: start as f32, 
                            end_time: stop as f32, 
                            skip_type: crate::structs::video::PlaylistEpisodeServerSkipType::OPENING 
                        }
                    )    
                }
            }
        }

        if let Ok(outro) = json.get("outro").as_object() {
            if let (Ok(start), Ok(stop)) = (outro.get("start").as_float(), outro.get("end").as_float()) {
                if stop > 0.0 {
                    skip_times.push(
                        PlaylistEpisodeServerSkipTime { 
                            start_time: start as f32, 
                            end_time: stop as f32, 
                            skip_type: crate::structs::video::PlaylistEpisodeServerSkipType::ENDING 
                        }
                    )    
                }
            }
        }

        Ok(
            PlaylistEpisodeServerResponse { 
                links, 
                subtitles,
                skip_times,
                headers: vec![],
            }
        )
    }
}