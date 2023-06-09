extern crate alloc;

use alloc::vec;
use alloc::{string::{String, ToString}, format, vec::Vec};
use mochi_imports::{http::{HTTPRequest, HTTPMethod}, error::Result, crypto::Crypto};
use mochi_structs::video::{PlaylistEpisodeServerResponse, PlaylistEpisodeServerFormatType, PlaylistEpisodeServerLink};

pub struct GogoCDN {}

struct Keys {
    key: String,
    second_key: String,
    iv: String
}

impl GogoCDN {
    pub fn extract(video_url: &str) -> Result<PlaylistEpisodeServerResponse> {
        let html = HTTPRequest::new(video_url, HTTPMethod::Get)
            .html()?;

        let keys = Keys {
            key: "37911490979715163134003223491201".to_string(),
            second_key: "54674138327930866480207815084989".to_string(),
            iv: "3134003223491201".to_string()
        };

        let id = video_url
            .split("id=")
            .last()
            .and_then(|e| e.split("&").nth(0))
            .unwrap_or_default();

        let id_bytes = id.as_bytes();

        let video_url_parts = video_url.split_once("//");
        let video_url_protocol = video_url_parts
            .map(|s| s.0)
            .unwrap_or_default();
        let video_url_hostname = video_url_parts
            .and_then(|s| s.1.split("/").nth(0))
            .unwrap_or_default();    

        let script_base64_string = html.select("script[data-name='episode']")
            .first()
            .attr("data-value");
        let script_bytes = Crypto::base64_parse(&script_base64_string);

        let decrypted_params_bytes = Crypto::aes_decrypt(&script_bytes, &keys.key, &keys.iv);
        let decrypted_params = String::from_utf8(decrypted_params_bytes)
            .unwrap_or_default();

        let encrypted_id_bytes = Crypto::aes_encrypt(id_bytes, &keys.key, &keys.iv);
        let encrypted_id = Crypto::base64_string(&encrypted_id_bytes);
        let encrypt_ajax_url = format!("{}//{}/encrypt-ajax.php?id={}&alias={}", video_url_protocol, video_url_hostname, encrypted_id, decrypted_params);

        let payload_json_string = HTTPRequest::new(
            &encrypt_ajax_url, 
            HTTPMethod::Get
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .string()?;

        let encrypted_data_encoded_string = payload_json_string
            // Cleanup invalid characters
            .trim_start_matches("{\"data\":\"")
            .trim_end_matches("\"}");
        let encryped_data_encoded_cleaned = encrypted_data_encoded_string
            // Cleanup invalid characters
            .replace("\\", "");
        let encrypted_data_bytes = Crypto::base64_parse(&encryped_data_encoded_cleaned);

        let decrypted_data_bytes = Crypto::aes_decrypt(&encrypted_data_bytes, &keys.second_key, &keys.iv);
        let decrypted_data_json = mochi_imports::json::parse(decrypted_data_bytes)
            .and_then(|o| o.as_object())?;

        let mut links: Vec<PlaylistEpisodeServerLink> = vec![];

        if let Ok(sources) = decrypted_data_json.get("source").as_array() {
            for source in sources {
                let link = source.as_object()
                    .and_then(|o| o.get("file").as_string());
                if let Ok(link) = link {
                    links.push(
                        PlaylistEpisodeServerLink { 
                            url: link, 
                            quality: mochi_structs::video::PlaylistEpisodeServerQualityType::Auto
                        }
                    );
                }
            }
        }

        if let Ok(sources) = decrypted_data_json.get("source_bk").as_array() {
            for source in sources {
                let link = source.as_object()
                    .and_then(|o| o.get("file").as_string());
                if let Ok(link) = link {
                    links.push(
                        PlaylistEpisodeServerLink { 
                            url: link, 
                            quality: mochi_structs::video::PlaylistEpisodeServerQualityType::Auto
                        }
                    );
                }
            }
        }

        Ok(
            PlaylistEpisodeServerResponse { 
                links, 
                subtitles: vec![], 
                format_type: PlaylistEpisodeServerFormatType::HLS 
            }
        )
    }
}