use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    page: Option<i32>,
    per_page: Option<i32>,
    season: Option<String>,
    season_year: Option<i32>,
    search: Option<String>,
}

impl Variables {
    pub fn new() -> Variables {
        Variables::default()
    }

    pub fn page_setup(&mut self, index: i32, length: i32) {
        self.page = Some(index);
        self.per_page = Some(length);
    }

    pub fn season_setup(&mut self, season: String, year: i32) {
        self.season = Some(season);
        self.season_year = Some(year);
    }

    pub fn search_setup(&mut self, search: String) {
        self.search = Some(search);
    }
}
