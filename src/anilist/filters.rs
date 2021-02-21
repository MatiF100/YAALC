use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
//Struct holding variables to be sent together with GraphQL query. Serialized to JSON, by serde_json
//If given field is Option::None, it is not included in serialized data
pub struct Variables {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id_mal: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    season: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    season_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<MediaType>,
}

impl Variables {
    //Creates new instance of Variables, with all fields set to None
    pub fn new() -> Variables {
        Variables::default()
    }

    //Setup of page index and entries per page
    pub fn page_setup(&mut self, index: i32, length: i32) {
        self.page = Some(index);
        self.per_page = Some(length);
    }

    //Setup of a given anime season
    pub fn season_setup(&mut self, season: String, year: i32) {
        self.season = Some(season);
        self.season_year = Some(year);
    }

    //Setup of "search" variable
    pub fn search_setup(&mut self, search: String) {
        if search.is_empty() {
            self.search = None;
            self.season_year = Some(2021);
        } else {
            self.search = Some(search);
        }
    }

    //Setting type of requested data to ANIME
    pub fn set_anime_type(&mut self) {
        self.media_type = Some(MediaType::ANIME);
    }
}

#[derive(Serialize, Deserialize, Debug)]
//Enum holding supported media types
pub enum MediaType {
    ANIME,
    MANGA,
}
