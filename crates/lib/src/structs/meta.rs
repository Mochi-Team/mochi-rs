extern crate alloc;

use alloc::string::ToString;
use alloc::vec;
use alloc::{string::String, vec::Vec};

use mochi_imports::error::Result;
use mochi_imports::core::{PtrRef, ArrayRef};

use super::conversion::optional_str_ptr;

#[link(wasm_import_module = "structs_meta")]
extern "C" {
    fn create_search_filter_option(
        option_id_ptr: i32,
        option_id_len: i32,
        name_ptr: i32, 
        name_len: i32
    ) -> i32;

    fn create_search_filter(
        id_ptr: i32, 
        id_len: i32, 
        name_ptr: i32, 
        name_len: i32, 
        options_arr_ref: i32,
        multiselect: bool,
        required: bool
    ) -> i32;

    // Create Paging
    fn create_paging(
        items_array_ref_ptr: i32,
        current_page_ptr: i32,
        current_page_len: i32,
        next_page_ptr: i32,
        next_page_len: i32
    ) -> i32;

    fn create_discover_listing(
        title_ptr: i32,
        title_len: i32,
        lising_type: i32,
        paging_ptr: i32
    ) -> i32;

    fn create_playlist(
        id_ptr: i32,
        id_len: i32,
        title_ptr: i32,
        title_len: i32,
        poster_image_ptr: i32,
        poster_image_len: i32,
        banner_image_ptr: i32,
        banner_image_len: i32,
        playlist_type: PlaylistType
    ) -> i32;

    fn create_playlist_details(
        description_ptr: i32,
        description_len: i32,
        alternative_titles_ptr: i32,
        alternative_posters_ptr: i32,
        alternative_banners_ptr: i32,
        genres_ptr: i32,
        year_released: i32,
        ratings: i32,
        previews_ptr: i32
    ) -> i32;

    fn create_playlist_preview(
        title_ptr: i32,
        title_len: i32,
        description_ptr: i32,
        description_len: i32,
        thumbnail_ptr: i32,
        thumbnail_len: i32,
        link_ptr: i32,
        link_len: i32,
        preview_type: PlaylistPreviewType
    ) -> i32;

    fn create_playlist_item(
        id_ptr: i32,
        id_len: i32,
        title_ptr: i32,
        title_len: i32,
        description_ptr: i32,
        description_len: i32,
        thumbnail_ptr: i32,
        thumbnail_len: i32,
        section_ptr: i32,
        section_len: i32,
        group: f64,
        number: f64,
        timestamp_ptr: i32,
        timestamp_len: i32,
        tags_ptr: i32
    ) -> i32;
}

pub trait Meta {
    fn search_filters() -> SearchFilters;
    fn search(search_query: SearchQuery) -> Result<Paging<Playlist>>;
    fn discovery_listing() -> Result<DiscoverListings>;
    fn playlist_details(id: String) -> Result<PlaylistDetails>;
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistType {
    Video,
    Image,
    Text
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: String,
    pub title: Option<String>,
    pub poster_image: Option<String>,
    pub banner_image: Option<String>,
    pub playlist_type: PlaylistType
}

#[derive(Debug, Clone)]
pub struct PlaylistDetails {
    pub description: Option<String>,
    pub alternative_titles: Vec<String>,
    pub alternative_posters: Vec<String>,
    pub alternative_banners: Vec<String>,
    pub genres: Vec<String>,
    pub year_released: Option<i32>,
    pub ratings: Option<i32>,
    pub previews: Vec<PlaylistPreview>
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlaylistPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: String,
    pub link: String,
    pub preview_type: PlaylistPreviewType
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistPreviewType {
    Video,
    Image
}

pub struct PlaylistItem {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub section: Option<String>,
    pub group: Option<f64>,
    pub number: f64,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Paging<T> {
    pub items: Vec<T>,
    pub current_page: String,
    pub next_page: Option<String>,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DiscoverListingType {
    Default,
    Rank,
    Featured
}

pub struct DiscoverListing {
    pub title: String,
    pub listing_type: DiscoverListingType,
    pub paging: Paging<Playlist>
}

pub struct DiscoverListings(pub Vec<DiscoverListing>);

pub struct SearchQuery {
    pub query: String,
    pub filters: Vec<SearchQueryFilter>,
    pub page: Option<String>
}

pub struct SearchQueryFilter {
    pub filter_id: String,
    pub option_id: String
}

pub struct SearchFilters {
    pub filters: Vec<SearchFilter>
}

pub struct SearchFilter {
    pub filter_id: String,
    pub display_name: String,
    pub options: Vec<SearchFilterOption>,
    pub multiselect: bool,
    pub required: bool
}

pub struct SearchFilterOption {
    pub option_id: String,
    pub display_name: String
}



impl From<SearchFilters> for PtrRef {
    fn from(value: SearchFilters) -> PtrRef {
        let mut array_ref = ArrayRef::new();

        for filter in value.filters {
            array_ref.insert(filter.into())
        }

        let filters_array_ref_ptr = array_ref.ptr();
        core::mem::forget(array_ref);
        Self::new(filters_array_ref_ptr)
    }
}

impl From<SearchFilter> for PtrRef {
    fn from(value: SearchFilter) -> Self {
        let mut array_ref = ArrayRef::new();

        for option in value.options {
            array_ref.insert(option.into())
        }

        let options_array_ref = array_ref.ptr();
        core::mem::forget(array_ref);

        let ptr = unsafe {
            create_search_filter(
                value.filter_id.as_ptr() as i32,
                value.filter_id.len() as i32,
                value.display_name.as_ptr() as i32,
                value.display_name.len() as i32,
                options_array_ref,
                value.multiselect,
                value.required
            )
        };
        Self::new(ptr)
    }
}

impl From<SearchFilterOption> for PtrRef {
    fn from(value: SearchFilterOption) -> Self {
        Self::new(
            unsafe {
                create_search_filter_option(
                    value.option_id.as_ptr() as i32,
                    value.option_id.len() as i32,
                    value.display_name.as_ptr() as i32,
                    value.display_name.len() as i32
                )
            }    
        )
    }
}

impl Into<SearchQuery> for PtrRef {
    fn into(self) -> SearchQuery {
        if self.is_some() {
            let reference = self.as_object();
            if let Ok(search_query_ref) = reference {
                let query = search_query_ref.get("query")
                    .as_string()
                    .unwrap_or_default();
                let mut filters: Vec<SearchQueryFilter> = Vec::new();
                if let Ok(filters_arr) = search_query_ref.get("filters").as_array() {
                    for item in filters_arr {
                        let filter_ref = match item.as_object() {
                            Ok(filter_ref) => filter_ref,
                            _ => continue,
                        };
                        let filter_id = match filter_ref.get("id").as_string() {
                            Ok(name) => name,
                            _ => continue,
                        };
                        let option_id = match filter_ref.get("optionId").as_string() {
                            Ok(value) => value,
                            _ => continue,
                        };
                        filters.push(
                            SearchQueryFilter { 
                                filter_id,
                                option_id
                            }
                        )
                    }
                }
                let page = search_query_ref.get("page").as_string().ok();
                return SearchQuery {
                    query,
                    filters,
                    page,
                }
            }
        }
        SearchQuery {
            query: "".to_string(),
            filters: vec![],
            page: None,
        }
    }
}

impl<T> From<Paging<T>> for PtrRef where PtrRef: From<T> {
    fn from(value: Paging<T>) -> Self {
        let mut items_array_ref = ArrayRef::new();

        for item in value.items {
            items_array_ref.insert(item.into())
        }

        let items_array_ref_ptr = items_array_ref.ptr();
        core::mem::forget(items_array_ref);

        let current_page = value.current_page;
        let next_page = optional_str_ptr(value.next_page);

        let obj_ptr = unsafe {
            create_paging(
                items_array_ref_ptr, 
                current_page.as_ptr() as i32, 
                current_page.len() as i32, 
                next_page.0, 
                next_page.1
            ) 
        };
        Self::new(obj_ptr)
    }
}

impl From<DiscoverListing> for PtrRef {
    fn from(value: DiscoverListing) -> Self {
        let title = value.title;
        let listing_type = value.listing_type;
        let paging: PtrRef = value.paging.into();
        let paging_ptr = paging.pointer();
        core::mem::forget(paging);

        let obj_ptr = unsafe {
            create_discover_listing(
                title.as_ptr() as i32, 
                title.len() as i32, 
                listing_type as i32,
                paging_ptr
            )
        };

        Self::new(obj_ptr)
    }
}

impl From<DiscoverListings> for PtrRef {
    fn from(value: DiscoverListings) -> Self {
        let mut array_ref = ArrayRef::new();

        for listing in value.0 {
            array_ref.insert(listing.into())
        }

        let array_ref_ptr = array_ref.ptr();
        core::mem::forget(array_ref);
        Self::new(array_ref_ptr)
    }
}

impl From<Playlist> for PtrRef {
    fn from(value: Playlist) -> Self {
        let id = value.id;
        let title = optional_str_ptr(value.title);
        let poster_image = optional_str_ptr(value.poster_image);
        let banner_image = optional_str_ptr(value.banner_image);
        let host_ptr = unsafe {
            create_playlist(
                id.as_ptr() as i32, 
                id.len() as i32,
                title.0,
                title.1,
                poster_image.0,
                poster_image.1,
                banner_image.0,
                banner_image.1,
                value.playlist_type
            )
        };
        Self::new(host_ptr)
    }
}

impl From<PlaylistDetails> for PtrRef {
    fn from(value: PlaylistDetails) -> Self {
        let description = optional_str_ptr(value.description);

        let mut alternative_titles = ArrayRef::new();
        for title in value.alternative_titles {
            alternative_titles.insert(title.into());
        }
        let alternative_titles_ptr = alternative_titles.ptr();
        core::mem::forget(alternative_titles);

        let mut alternative_posters = ArrayRef::new();
        for poster in value.alternative_posters {
            alternative_posters.insert(poster.into());
        }
        let alternative_posters_ptr = alternative_posters.ptr();
        core::mem::forget(alternative_posters);

        let mut alternative_banners = ArrayRef::new();
        for banner in value.alternative_banners {
            alternative_banners.insert(banner.into());
        }
        let alternative_banners_ptr = alternative_banners.ptr();
        core::mem::forget(alternative_banners);

        let mut genres = ArrayRef::new();
        for genre in value.genres {
            genres.insert(genre.into());
        }
        let genres_ptr = genres.ptr();
        core::mem::forget(genres);

        let mut previews = ArrayRef::new();
        for preview in value.previews {
            previews.insert(preview.into());
        }
        let previews_ptr = previews.ptr();
        core::mem::forget(previews);

        let host_ptr = unsafe {
            create_playlist_details(
                description.0, 
                description.1, 
                alternative_titles_ptr, 
                alternative_posters_ptr,
                alternative_banners_ptr, 
                genres_ptr, 
                value.year_released.unwrap_or(-1), 
                value.ratings.unwrap_or(-1), 
                previews_ptr
            )
        };

        Self::new(host_ptr)
    }
}

impl From<PlaylistPreview> for PtrRef {
    fn from(value: PlaylistPreview) -> Self {
        let title = optional_str_ptr(value.title);
        let description = optional_str_ptr(value.description);

        let host_ptr = unsafe {
            create_playlist_preview(
                title.0, 
                title.1, 
                description.0, 
                description.1, 
                value.thumbnail.as_ptr() as i32, 
                value.thumbnail.len() as i32,
                value.link.as_ptr() as i32,
                value.link.len() as i32, 
                value.preview_type
            )
        };
        PtrRef::from(host_ptr)
    }
}

impl From<PlaylistItem> for PtrRef {
    fn from(value: PlaylistItem) -> Self {
        let title = optional_str_ptr(value.title);
        let description = optional_str_ptr(value.description);
        let thumbnail = optional_str_ptr(value.thumbnail);
        let section = optional_str_ptr(value.section);
        let timestamp = optional_str_ptr(value.timestamp);

        let mut tags = ArrayRef::new();

        for tag in value.tags {
            tags.insert(tag.into());
        }

        let tags_ptr = tags.ptr();
        core::mem::forget(tags);

        let host_ptr = unsafe {
            create_playlist_item(
                value.id.as_ptr() as i32,
                value.id.len() as i32,
                title.0,
                title.1,
                description.0,
                description.1,
                thumbnail.0,
                thumbnail.1,
                section.0,
                section.1,
                value.group.unwrap_or(-1.0),
                value.number, 
                timestamp.0, 
                timestamp.1, 
                tags_ptr
            )
        };
        Self::from(host_ptr)
    }
}