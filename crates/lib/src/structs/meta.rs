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

    fn create_media(
        id_ptr: i32,
        id_len: i32,
        title_ptr: i32,
        title_len: i32,
        poster_image_ptr: i32,
        poser_image_len: i32,
        banner_image_ptr: i32,
        banner_image_len: i32,
        meta: MediaMeta
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
}

pub trait Meta {
    fn search_filters() -> SearchFilters;
    fn search(search_query: SearchQuery) -> Result<Paging<Media>>;
    fn discovery_listing() -> Result<DiscoverListings>;
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MediaMeta {
    Video,
    Image,
    Text
}

#[derive(Debug, Clone)]
pub struct Media {
    pub id: String,
    pub title: Option<String>,
    pub poster_image: Option<String>,
    pub banner_image: Option<String>,
    pub meta: MediaMeta
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
    pub paging: Paging<Media>
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

impl From<Media> for PtrRef {
    fn from(value: Media) -> Self {
        let id = value.id;
        let title = optional_str_ptr(value.title);
        let poster_image = optional_str_ptr(value.poster_image);
        let banner_image = optional_str_ptr(value.banner_image);
        let host_ptr = unsafe {
            create_media(
                id.as_ptr() as i32, 
                id.len() as i32,
                title.0,
                title.1,
                poster_image.0,
                poster_image.1,
                banner_image.0,
                banner_image.1,
                value.meta
            )
        };
        Self::new(host_ptr)
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