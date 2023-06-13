extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use mochi_imports::crypto::Crypto;
use mochi_imports::error::Result;
use mochi_imports::http::HTTPMethod;
use mochi_imports::http::HTTPRequest;
use mochi_structs::video::PlaylistEpisodeServerLink;
use mochi_structs::video::PlaylistEpisodeServerResponse;
use mochi_structs::video::PlaylistEpisodeServerSubtitle;

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
            .expect("Id could not be found for rapidcloud link.");

        let url = format!("{}/ajax/embed-6/getSources?id={}", HOST, id);

        let json = HTTPRequest::new(
            &url, 
            HTTPMethod::Get
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
            let encrypted_key = HTTPRequest::new(
                // TODO: Add param at the end so we don't cache request
                "https://raw.githubusercontent.com/enimax-anime/key/e6/key.txt", 
                HTTPMethod::Get
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

            let parsed_array = mochi_imports::json::parse(decrypted)?
                .as_array()?;

            for item in parsed_array {
                let object = item.as_object()?;
                if let Ok(link) = object.get("file").as_string() {
                    links.push(
                        PlaylistEpisodeServerLink { 
                            url: link, 
                            quality: mochi_structs::video::PlaylistEpisodeServerQualityType::Auto,
                            format_type: mochi_structs::video::PlaylistEpisodeServerFormatType::HLS
                        }
                    )
                }
            }
        }

        let mut subtitles: Vec<PlaylistEpisodeServerSubtitle> = vec![];

        if let Ok(tracks_elements) = json.get("tracks").as_array() {
            for element in tracks_elements {
                let track = element.as_object()?;
                let file = track.get("file").as_string()?;
                let label = track.get("label").as_string()?;
                let kind = track.get("kind").as_string();
                if let Ok(kind) = kind {
                    if kind.contains("captions") {
                        subtitles.push(
                            PlaylistEpisodeServerSubtitle { 
                                url: file, 
                                language: label, 
                                format: mochi_structs::video::PlaylistEpisodeServerSubtitleFormat::SRT
                            }
                        )
                    }
                }
            }    
        }

        // TODO: Add timestamps

        Ok(
            PlaylistEpisodeServerResponse { 
                links, 
                subtitles
            }
        )
        // Err(MochiError::Unknown)
    }
}