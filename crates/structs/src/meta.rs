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

    fn create_paging(
        id_ptr: i32,
        id_len: i32,
        previous_page_ptr: i32,
        previous_page_len: i32,
        next_page_ptr: i32,
        next_page_len: i32,
        items_ptr: i32
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
        number: f64,
        timestamp_ptr: i32,
        timestamp_len: i32,
        tags_ptr: i32
    ) -> i32;

    fn create_playlist_items_response(
        contents_ptr: i32,
        all_groups_ptr: i32
    ) -> i32;

    fn create_playlist_group(
        id: f64,
        display_title_ptr: i32,
        display_title_len: i32
    ) -> i32;

    fn create_playlist_group_page(
        id_ptr: i32,
        id_len: i32,
        display_name_ptr: i32,
        display_name_len: i32
    ) -> i32;

    fn create_playlist_group_items(
        group_id: f64,
        pagings_ptr: i32,
        all_pages_ptr: i32
    ) -> i32;
}

pub trait Meta {
    fn search_filters() -> SearchFilters;
    fn search(search_query: SearchQuery) -> Result<Paging<Playlist>>;
    fn discover_listings() -> Result<DiscoverListings>;
    fn playlist_details(id: String) -> Result<PlaylistDetails>;
}

pub type PlaylistID = String;

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: PlaylistID,
    pub title: Option<String>,
    /// Valid URL Image
    pub poster_image: Option<String>,
    /// Valid URL Image
    pub banner_image: Option<String>,
    /// Valid url string
    pub url: String,
    /// Indicates the status of the playlist
    pub status: PlaylistStatus,
    pub playlist_type: PlaylistType
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistStatus {
    Unknown,
    Upcoming,
    Ongoing,
    Completed,
    Paused,
    Cancelled,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistType {
    Video,
    Image,
    Text
}

#[derive(Debug, Clone)]
pub struct PlaylistDetails {
    pub description: Option<String>,
    pub alternative_titles: Vec<String>,
    /// array of valid urls
    pub alternative_posters: Vec<String>,
    /// array of valid urls
    pub alternative_banners: Vec<String>,
    pub genres: Vec<String>,
    pub year_released: Option<i32>,
    /// 0...10
    pub ratings: Option<i32>,
    pub previews: Vec<PlaylistPreview>
}

#[derive(Debug, Clone)]
pub struct PlaylistPreview {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    /// Must be a valid url string
    pub link: String,
    pub preview_type: PlaylistPreviewType
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlaylistPreviewType {
    Video,
    Image
}

pub type PlaylistItemID = String;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PlaylistItem {
    pub id: PlaylistItemID,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub number: f64,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlaylistItemsRequest {
    pub playlist_id: PlaylistID,
    pub group_id: Option<f64>,
    pub page_id: Option<String>,
    pub item_id: Option<String>
}

#[derive(Debug, Clone)]
pub struct PlaylistItemsResponse {
    pub contents: Vec<PlaylistGroupContent>,
    pub all_groups: Vec<PlaylistGroup>
}

#[derive(Debug, Clone)]
pub struct PlaylistGroup {
    pub id: f64,
    pub display_title: Option<String>
}

#[derive(Debug, Clone)]
pub struct PlaylistGroupPage {
    pub id: String,
    pub display_name: String
}

#[derive(Debug, Clone)]
pub struct PlaylistGroupContent {
    pub group_id: f64,
    pub pagings: Vec<Paging<PlaylistItem>>,
    pub all_pages: Vec<PlaylistGroupPage>
}

#[derive(Debug, Clone)]
pub struct Paging<T> {
    pub id: String,
    pub previous_page: Option<String>,
    pub next_page: Option<String>,
    pub items: Vec<T>
}

#[derive(Debug, Clone)]
pub struct DiscoverListing {
    pub title: String,
    pub listing_type: DiscoverListingType,
    pub paging: Paging<Playlist>
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DiscoverListingType {
    Default,
    Rank,
    Featured
}

#[derive(Debug, Clone)]
pub struct DiscoverListings(pub Vec<DiscoverListing>);

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub filters: Vec<SearchQueryFilter>,
    pub page: Option<String>
}

#[derive(Debug, Clone)]
pub struct SearchQueryFilter {
    pub filter_id: String,
    pub option_id: String
}

#[derive(Debug, Clone)]
pub struct SearchFilters {
    pub filters: Vec<SearchFilter>
}

#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub filter_id: String,
    pub display_name: String,
    pub options: Vec<SearchFilterOption>,
    pub multiselect: bool,
    pub required: bool
}

#[derive(Debug, Clone)]
pub struct SearchFilterOption {
    pub option_id: String,
    pub display_name: String
}

impl From<SearchFilters> for PtrRef {
    fn from(value: SearchFilters) -> PtrRef {
        let array_ref = ArrayRef::from(value.filters);
        let filters_array_ref_ptr = array_ref.ptr();
        core::mem::forget(array_ref);
        Self::new(filters_array_ref_ptr)
    }
}

impl From<SearchFilter> for PtrRef {
    fn from(value: SearchFilter) -> Self {
        let array_ref = ArrayRef::from(value.options);
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
        let items = ArrayRef::from(value.items);
        let items_ptr = items.ptr();
        core::mem::forget(items);

        let previous_page = optional_str_ptr(value.previous_page);
        let next_page = optional_str_ptr(value.next_page);

        let obj_ptr = unsafe {
            create_paging(
                value.id.as_ptr() as i32, 
                value.id.len() as i32, 
                previous_page.0, 
                previous_page.1, 
                next_page.0,
                next_page.1,
                items_ptr
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
        let array_ref = ArrayRef::from(value.0);
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

        let alternative_titles = ArrayRef::from(value.alternative_titles);
        let alternative_titles_ptr = alternative_titles.ptr();
        core::mem::forget(alternative_titles);

        let alternative_posters = ArrayRef::from(value.alternative_posters);
        let alternative_posters_ptr = alternative_posters.ptr();
        core::mem::forget(alternative_posters);

        let alternative_banners = ArrayRef::from(value.alternative_banners);
        let alternative_banners_ptr = alternative_banners.ptr();
        core::mem::forget(alternative_banners);

        let genres = ArrayRef::from(value.genres);
        let genres_ptr = genres.ptr();
        core::mem::forget(genres);

        let previews = ArrayRef::from(value.previews);
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

        let thumbnail = optional_str_ptr(value.thumbnail);

        let host_ptr = unsafe {
            create_playlist_preview(
                title.0, 
                title.1, 
                description.0, 
                description.1, 
                thumbnail.0, 
                thumbnail.1,
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
        let timestamp = optional_str_ptr(value.timestamp);

        let tags = ArrayRef::from(value.tags);
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
                value.number, 
                timestamp.0, 
                timestamp.1, 
                tags_ptr
            )
        };
        Self::from(host_ptr)
    }
}

impl Into<PlaylistItemsRequest> for PtrRef {
    fn into(self) -> PlaylistItemsRequest {
        if let Ok(reference) = self.as_object() {
            let playlist_id = reference.get("playlistId")
                .as_string()
                .unwrap_or_default();

            let group_id = reference.get("groupId").as_float().ok();
            let page_id = reference.get("pageId").as_string().ok();
            let item_id = reference.get("itemId").as_string().ok();

            PlaylistItemsRequest {
                playlist_id,
                group_id,
                page_id,
                item_id,
            }
        } else {
            PlaylistItemsRequest {
                playlist_id: "".to_string(),
                group_id: None,
                page_id: None,
                item_id: None,
            }
        }
    }
}

impl From<PlaylistItemsResponse> for PtrRef {
    fn from(value: PlaylistItemsResponse) -> Self {
        let contents = ArrayRef::from(value.contents);
        let contents_ptr = contents.ptr();
        core::mem::forget(contents);

        let all_groups = ArrayRef::from(value.all_groups);
        let all_groups_ptr = all_groups.ptr();
        core::mem::forget(all_groups);

        Self::new(unsafe { create_playlist_items_response(contents_ptr, all_groups_ptr) } )
    }
}

impl From<PlaylistGroup> for PtrRef {
    fn from(value: PlaylistGroup) -> Self {
        let display_title = optional_str_ptr(value.display_title);
        Self::new(
            unsafe {
                create_playlist_group(
                    value.id, 
                    display_title.0,
                    display_title.1
                )
            }
        )
    }
}

impl From<PlaylistGroupPage> for PtrRef {
    fn from(value: PlaylistGroupPage) -> Self {
        Self::new(
            unsafe {
                create_playlist_group_page(
                    value.id.as_ptr() as i32,
                    value.id.len() as i32,
                    value.display_name.as_ptr() as i32,
                    value.display_name.len() as i32
                )
            }
        )
    }
}

impl From<PlaylistGroupContent> for PtrRef {
    fn from(value: PlaylistGroupContent) -> Self {
        let pagings = ArrayRef::from(value.pagings);
        let pagings_ptr = pagings.ptr();
        core::mem::forget(pagings);

        let all_pages = ArrayRef::from(value.all_pages);
        let all_pages_ptr = all_pages.ptr();
        core::mem::forget(all_pages);

        Self::new(
            unsafe {
                create_playlist_group_items(
                    value.group_id, 
                    pagings_ptr,
                    all_pages_ptr
                )
            }
        )
    }
}