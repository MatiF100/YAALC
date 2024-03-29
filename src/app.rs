use crate::anilist;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::time::SystemTime;
use tui::widgets::ListState;

//Struct holding information about current app state, including current mode and data to be shown
#[derive(Default, Debug)]
pub struct App {
    pub title: String,
    pub user: Option<User>,
    pub animes: StatefulList<Media>,
    pub token: Option<AuthToken>,
    pub should_exit: bool,
    pub legend: Vec<(String, String)>,
    pub mode: AppMode,
    pub search_bar: String,
}

impl App {
    //Creating new app backend instance
    pub fn new(title: String) -> App {
        App {
            title: title,
            animes: StatefulList::new(),
            should_exit: false,
            ..Default::default()
        }
    }

    pub fn set_legend(&mut self, legend: Vec<(String, String)>) {
        self.legend = legend;
    }

    //Getting authorization token from the anilist.co, or reading it from file
    pub async fn authorize(&mut self) {
        match std::fs::read_to_string("token.json") {
            Ok(token) => {
                let token: AuthToken = serde_json::from_str(&token).unwrap();
                if AuthToken::validate_token(&token) {
                    self.token = Some(token);
                } else {
                    self.token = AuthToken::request_and_save_token();
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    self.token = AuthToken::request_and_save_token();
                }
                _ => println!(
                    "Failed to read authentication token! App will run in search_only mode"
                ),
            },
        }
        if let &Some(_) = &self.token {
            self.user = self.get_current_user_data().await;
        }
        //self.token = anilist::auth::auth();
    }

    //Retrieving the auth token for use in code
    pub fn get_token(&self) -> Option<String> {
        match &self.token {
            Some(token) => Some(token.get_token()),
            None => None,
        }
    }

    //Loading into app animes that meet name criteria
    async fn search_animes(&mut self, search: String) {
        self.animes = StatefulList::with_items(anilist::search_anime_by_name(search, &self).await);
    }

    async fn get_current_user_data(&self) -> Option<User> {
        //println!("{:?}",serde_json::from_value::<User>(anilist::send_request(&self, anilist::queries::CURRENT_USER_DATA, anilist::filters::Variables::new()).await));
        if let Ok(response) = serde_json::from_value::<RecievedData<RecievedUser>>(
            anilist::send_request(
                &self,
                anilist::queries::CURRENT_USER_DATA,
                anilist::filters::Variables::new(),
            )
            .await,
        ) {
            if let Some(data) = response.data {
                data.user
            } else {
                None
            }
        } else {
            None
        }
    }

    //Listener for keyboard input handling. Actions are dependant on AppMode
    pub async fn handle_input(&mut self, input: Event) {
        match self.mode {
            AppMode::NORMAL => {
                match input {
                    Event::Key(key) => match key {
                        KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: _,
                        } => {
                            //terminal::leave_terminal();
                            self.should_exit = true;
                        }
                        KeyEvent {
                            code: KeyCode::Char('i'),
                            modifiers: _,
                        } => {
                            self.mode = AppMode::INPUT;
                        }
                        KeyEvent {
                            code: KeyCode::Up,
                            modifiers: _,
                        } => self.animes.previous(),
                        KeyEvent {
                            code: KeyCode::Down,
                            modifiers: _,
                        } => self.animes.next(),
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: _,
                        } => self.animes.unselect(),
                        _ => (),
                    },
                    //Event::Resize(_, _) => terminal::draw_frame(&mut terminal, &mut app),
                    _ => (),
                }
            }
            AppMode::INPUT => match input {
                Event::Key(key) => match key {
                    KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: KeyModifiers::SHIFT,
                    } => self.search_bar = "".to_owned(),
                    KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: _,
                    } => {
                        self.mode = AppMode::NORMAL;
                        self.search_bar = "".to_owned()
                    }
                    KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: _,
                    } => self.search_bar.push_str(String::from(c).as_ref()),
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: _,
                    } => {
                        self.search_bar.pop();
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: _,
                    } => {
                        self.search_animes(self.search_bar.clone()).await;
                        self.mode = AppMode::NORMAL;
                    }
                    _ => (),
                },
                _ => (),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MediaListCollection{
    lists: MediaListGroup,
    has_next_chunk: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MediaListGroup{
    entries: Vec<MediaList>,
    name: String,
    is_custom_list: bool,
    is_split_completed_list: bool,
    status: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct MediaList{
    id: i32,
    user_id: i32,
    media_id: i32,
    status: String,
    score: f32,
    progress: i32,
    media: Media,
    user: Option<User>
}

//Struct holding information about currently authenticated user
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Viewer")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub statistics: UserStatisticTypes,
}

//Struct holding User field contained in data recieved from anilist.co. Written as to allow serialization and deserialization using serde library
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RecievedUser {
    #[serde(alias = "Viewer")]
    pub user: Option<User>,
}

//Struct holding information about user's statistics in media
#[derive(Serialize, Deserialize, Debug)]
pub struct UserStatisticTypes {
    pub anime: UserStatistics,
    pub manga: UserStatistics,
}

//Struct holding information about given medium statistic
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
    pub count: i32,
    pub mean_score: f32,
    pub standard_deviation: f32,
    pub minutes_watched: i32,
    pub episodes_watched: i32,
    pub chapters_read: i32,
    pub volumes_read: i32,
}

//Struct holding information about authorization token
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    access_token: AccessToken,
    expires_at: u64,
}
impl AuthToken {
    //Saving the token and its expiration time as unix timestamp
    pub fn from_args(token: AccessToken, expires: u64) -> AuthToken {
        AuthToken {
            access_token: token,
            expires_at: expires
                + SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
        }
    }

    //Retrieving saved token
    pub fn get_token(&self) -> String {
        let mut token = String::from("Bearer ");
        token.push_str(self.access_token.secret());
        token
    }
    //Function checking if given token is still valid
    fn validate_token(token: &AuthToken) -> bool {
        if token.expires_at
            <= SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 7200
        {
            return false;
        }
        true
    }

    fn request_and_save_token() -> Option<AuthToken> {
        let token = anilist::auth::auth();
        match serde_json::to_writer_pretty(&File::create("token.json").unwrap(), &token) {
            Err(e) => println!("Unexpected error! Failed to save auth token! {}", e),
            _ => (),
        };
        token
    }
}

//Enum containing possible application states.
#[derive(Debug)]
pub enum AppMode {
    NORMAL,
    INPUT,
}

impl Default for AppMode {
    //By default app starts in NORMAL mode
    fn default() -> AppMode {
        AppMode::NORMAL
    }
}

//Struct holding data and/or errors recieved from anilist.co. Written as to allow serialization and deserialization using serde library
#[derive(Serialize, Deserialize, Debug)]
pub struct RecievedData<T> {
    pub data: Option<T>,
    pub errors: Option<serde_json::Value>,
}

//Struct holding Page field contained in data recieved from anilist.co. Written as to allow serialization and deserialization using serde library
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RecievedPage {
    pub page: Option<PagedMedia>,
}

//Struct holding contents of Page field in data recieved from anilist.co. Written as to allow serialization and deserialization using serde library
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PagedMedia {
    pub page_info: PageDetails,
    pub media: Vec<Media>,
}

//Struct holding details of page recieved from anilist.co, such as it's index, total number of pages and more. Written as to allow serialization and deserialization using serde library
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageDetails {
    pub total: Option<i32>,
    pub per_page: Option<i32>,
    pub current_page: Option<i32>,
    pub last_page: Option<i32>,
    pub has_next_page: Option<bool>,
}

//Struct holding data about anime recieved from anilist.co. Written as to allow serialization and deserialization using serde library
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
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

//Struct holding information about the anime's title
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Title {
    pub native: Option<String>,
    pub romaji: Option<String>,
    pub english: Option<String>,
}

impl Title {
    //Retrieving the most convenient title to display in order: English>Romaji>Native
    pub fn get_title(&self) -> String {
        match &self.english {
            Some(title) => title.to_string(),
            None => match &self.romaji {
                Some(title) => title.to_string(),
                None => match &self.native {
                    Some(title) => title.to_string(),
                    None => "Missing Title".to_owned(),
                },
            },
        }
    }
}

#[derive(Debug)]
//Struct holding some data as vector, as well as information about currently selected element
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
