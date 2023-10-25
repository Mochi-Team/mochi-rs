extern crate alloc;

use alloc::{format, vec, vec::Vec, string::String};
use crate::structs::video::{PlaylistEpisodeServerResponse, PlaylistEpisodeServerSubtitle, PlaylistEpisodeServerSubtitleFormat, PlaylistEpisodeServerLink, PlaylistEpisodeServerHeader};
use crate::imports::{error::Result, http::{Request, RequestMethod}, crypto::Crypto};

static HOST: &'static str = "https://dokicloud.one";
static HOST2: &'static str = "https://rabbitstream.net";

pub struct VidCloud {}

impl VidCloud {
    pub fn extract(
        video_url: &str,
        is_variant: bool
    ) -> Result<PlaylistEpisodeServerResponse> {
        let id = video_url
            .split("/")
            .last()
            .expect("video url does not have id")
            .split_once("?")
            .map(|o| o.0)
            .expect("video url does not have id");

        let host_url = if is_variant { HOST2 } else { HOST };

        let response = Request::new(
            &format!("{}/ajax/embed-4/getSources?id={}", host_url, id), 
            RequestMethod::Get
        )
        .header("X-Requested-With", "XMLHttpRequest",)
        .header("Referer", video_url)
        .json()?
        .as_object()?;

        let encrypted = response.get("encrypted")
            .as_bool()
            .unwrap_or_default();

        let sources;

        if encrypted {
            let encrypted_key: Vec<Vec<i64>> = Request::new(
                "https://raw.githubusercontent.com/enimax-anime/key/e4/key.txt", 
                RequestMethod::Get
            )
            .json()
            .expect("json array of nested ints")
            .as_array()?
            .map(|e| {
                e.as_array()
                    .expect("nested array of ints")
                    .map(|n| n.as_int().expect("int value"))
                    .collect()
            })
            .collect();

            let mut key: Vec<u8> = vec![];

            let mut encrypted_sources = response.get("sources")
                .as_string()
                .unwrap_or_default()
                .as_bytes()
                .to_vec();

            for i in encrypted_key {
                for j in i[0]..i[1] {
                    key.push(encrypted_sources[j as usize]);
                    encrypted_sources[j as usize] = b' ';
                }
            }

            encrypted_sources.retain(|x| x != &b' ');

            let encrypted_bytes = Crypto::base64_parse(&String::from_utf8(encrypted_sources).unwrap_or_default());
            let encrypted_salt = encrypted_bytes[8..16].to_vec();

            let mut encrypted_key_and_salt = key;
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
                &encrypted_bytes[16..], 
                &hashed_key_and_iv[0..32], 
                &hashed_key_and_iv[32..48]
            );

            sources = crate::imports::json::parse(decrypted)?.as_array()?;
        } else {
            sources = response.get("sources").as_array()?;
        }

        let mut links: Vec<PlaylistEpisodeServerLink> = vec![];

        for source in sources {
            let object = source.as_object()?;
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

        let mut subtitles: Vec<PlaylistEpisodeServerSubtitle> = vec![];

        if let Ok(elements) = response.get("tracks").as_array() {
            for element in elements {
                if let Ok(track) = element.as_object() {
                    let file = track.get("file").as_string()?;
                    let title = track.get("label").as_string().unwrap_or_default();
                    let default = track.get("default").as_bool().unwrap_or(false);
                    subtitles.push(
                        PlaylistEpisodeServerSubtitle { 
                            url: file, 
                            name: title, 
                            format: PlaylistEpisodeServerSubtitleFormat::VTT, 
                            default, 
                            autoselect: default
                        }
                    )
                }
            }
        }

        Ok(
            PlaylistEpisodeServerResponse {
                links,
                subtitles,
                skip_times: vec![],
                headers: vec![
                    PlaylistEpisodeServerHeader {
                        key: "Referer".into(),
                        value: video_url.into()
                    }
                ]
            }
        )
    }
}