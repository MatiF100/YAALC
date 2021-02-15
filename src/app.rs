use tui::widgets::ListState;

pub struct App {
    pub title: String,
    //pub animes: StatefulList<Anime>,
    pub animes: StatefulList<String>,
}

impl App {
    pub fn new(title: String) -> App {
        App {
            title: title,
            animes: StatefulList::new(),
        }
    }
}

pub struct Anime {
    pub id: i32,
    pub id_mal: i32,
    pub season: String,
    pub season_year: i32,
    pub episodes: i32,
    pub genre: String,
}

impl Anime {
    pub fn from_json() {}
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
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
