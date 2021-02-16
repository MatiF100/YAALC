use serde::{Deserialize, Serialize};
use tui::widgets::ListState;

#[derive(Default)]
pub struct App {
    pub title: String,
    pub animes: StatefulList<Anime>,
    pub should_exit: bool,
    pub legend: Vec<(String, String)>,
}

impl App {
    pub fn new(title: String) -> App {
        App {
            title: title,
            animes: StatefulList::new(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecievedData {
    pub data: Option<RecievedPage>,
    pub errors: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RecievedPage {
    pub page: Option<PagedAnime>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagedAnime {
    pub page_info: Option<PageDetails>,
    pub media: Option<Vec<Anime>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageDetails {
    pub total: Option<i32>,
    pub per_page: Option<i32>,
    pub current_page: Option<i32>,
    pub last_page: Option<i32>,
    pub has_next_page: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    pub id: Option<i32>,
    pub id_mal: Option<i32>,
    pub title: Title,
    pub season: Option<String>,
    pub season_year: Option<i32>,
    pub episodes: Option<i32>,
    pub genres: Option<Vec<String>>,
    pub status: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Title {
    pub native: Option<String>,
    pub romaji: Option<String>,
    pub english: Option<String>,
}

impl Title{
    pub fn get_title(&self) -> String{
        match &self.english{
            Some(title) => title.to_string(),
            None => match &self.romaji{
                Some(title) => title.to_string(),
                None => match &self.native{
                    Some(title) => title.to_string(),
                    None => "Missing Title".to_owned()
                }
            }
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
