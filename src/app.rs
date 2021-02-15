use serde::{Deserialize, Serialize};
use tui::widgets::ListState;

#[derive(Default)]
pub struct App {
    pub title: String,
    //pub animes: StatefulList<Anime>,
    pub animes: StatefulList<String>,
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
#[serde(rename_all = "PascalCase")]
pub struct RecievedData{
    page: Option<PagedAnime>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagedAnime{
    page_info: Option<PageDetails>,
    media: Option<Vec<Anime>>

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageDetails{
    total: Option<i32>,
    per_page: Option<i32>,
    current_page: Option<i32>,
    last_page: Option<i32>,
    has_next_page: Option<bool>
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
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Title{
    native: Option<String>,
    romaji: Option<String>,
    english: Option<String>
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
