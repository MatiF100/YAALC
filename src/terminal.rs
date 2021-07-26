use crossterm::{
    event::{read, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget, Wrap};
use tui::Frame;
use tui::Terminal;

//use serde_json::json;

use crate::app::{App, AppMode, StatefulList};

//Entering alternate screen, and creating new terminal instance
//Returning handle to this terminal
pub fn create_terminal() -> Terminal<CrosstermBackend<std::io::Stdout>> {
    execute!(io::stdout(), EnterAlternateScreen);

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    match Terminal::new(backend) {
        Ok(k) => k,
        Err(e) => panic!("Unexpected error happened: {}", e),
    }
}

pub fn process_event(event: Event) {}

//Leaving the alternate screen
pub fn leave_terminal() {
    execute!(io::stdout(), LeaveAlternateScreen);
}

//Entering the alternate screen
#[allow(dead_code)]
pub fn reenter_terminal() {
    execute!(io::stdout(), EnterAlternateScreen);
}

//Creating thread litening for user input events
//Returns  mpsc::Reciever, that recieves queued inputs
pub fn events_test() -> std::sync::mpsc::Receiver<Event> {
    let (tx, rx) = channel();

    thread::spawn(move || loop {
        tx.send(read().unwrap()).unwrap();
    });
    rx
}

//Drawing basic terminal layout
//Recieves mutable reference to terminal handle, and mutable reference to struct holding informatio about current app state
pub fn draw_frame(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
    match terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(8),
                    Constraint::Percentage(85),
                    Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .split(f.size());
        let main_areas = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[1]);

        let search_block = Block::default().title("Search").borders(Borders::ALL);
        let search_text = match app.mode {
            AppMode::INPUT => format!("{}_", app.search_bar),
            AppMode::NORMAL => app.search_bar.clone(),
        };
        let anime_list = Paragraph::new(search_text)
            .block(search_block)
            .wrap(Wrap { trim: false });
        f.render_widget(anime_list, chunks[0]);
        draw_list(f, main_areas[0], app);
        draw_details(f, main_areas[1], app);
        draw_legend(f, chunks[2], app);
    }) {
        Ok(_) => (),
        Err(e) => panic!("Unexpected error happened: {}", e),
    };
}

fn draw_details(
    f: &mut Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    area: Rect,
    app: &mut App,
) {
    let selected_anime = app.animes.state.selected();

    match selected_anime {
        Some(index) => {
            let selected_anime = match app.animes.items.get(index) {
                Some(anime) => anime,
                None => {
                    let info = Paragraph::new("Unexpected error happened! Please contact app creator, together with error information: Error while indexing into anime list")
                    .block(Block::default().title("Details").borders(Borders::ALL))
                    .wrap(Wrap{trim: true});
                    f.render_widget(info, area);
                    return;
                }
            };

            let formatted_details = vec![
                Spans::from(Span::styled(
                    "TITLE",
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )),
                Spans::from(vec![
                    Span::from("Native: "),
                    Span::from(unpack_detail(selected_anime.title.native.as_ref())),
                ]),
                Spans::from(vec![
                    Span::from("Romaji: "),
                    Span::from(unpack_detail(selected_anime.title.romaji.as_ref())),
                ]),
                Spans::from(vec![
                    Span::from("English: "),
                    Span::from(unpack_detail(selected_anime.title.english.as_ref())),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "Airing season",
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::from(": "),
                    Span::from(unpack_detail(selected_anime.season.as_ref())),
                    Span::from(" "),
                    Span::from(unpack_detail(selected_anime.season_year.as_ref())),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "Total episodes",
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::from(": "),
                    Span::from(unpack_detail(selected_anime.episodes.as_ref())),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "Airing status",
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::from(": "),
                    Span::from(unpack_detail(selected_anime.status.as_ref())),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "Episode duration (minutes)",
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::from(": "),
                    Span::from(unpack_detail(selected_anime.duration.as_ref())),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "Genres",
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::from(": "),
                    Span::from(unpack_vector(selected_anime.genres.as_ref())),
                ]),
            ];
            let info = Paragraph::new(formatted_details)
                .block(Block::default().title("Details").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(info, area)
        }
        None => {
            let info = Paragraph::new(
                "Please select an anime from the list, or search for something more interesting",
            )
            .block(Block::default().title("Details").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
            f.render_widget(info, area);
        }
    }
}

fn unpack_detail<T: std::fmt::Display>(detail: Option<T>) -> String {
    match detail {
        Some(data) => data.to_string(),
        None => String::from("Unknown"),
    }
}

fn unpack_vector(details: Option<&Vec<String>>) -> String {
    let mut output = String::new();
    match details {
        Some(v) => {
            for element in v {
                output.push_str("\"");
                output.push_str(element.as_ref());
                output.push_str("\"");
                output.push_str(" ");
            }
        }
        None => output = String::from("No information"),
    }
    output
}

//Function used to draw list of animes contained in App struct
fn draw_list(
    f: &mut Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    area: Rect,
    app: &mut App,
) {
    let tasks: Vec<ListItem> = app
        .animes
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.title.get_title()))]))
        .collect();
    let tasks = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Anime"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(" >");

    //f.render_widget(tasks.clone(), chunks[0]);
    f.render_stateful_widget(tasks, area, &mut app.animes.state);
    //println!("{:?}", app.animes.state);
}

fn draw_legend(
    f: &mut Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
    area: Rect,
    app: &mut App,
) {
    let binds = app
        .legend
        .iter()
        .map(|tuple| {
            vec![
                Span::styled(
                    tuple.0.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    ": ".to_owned(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    tuple.1.clone(),
                    Style::default().add_modifier(Modifier::UNDERLINED),
                ),
                Span::raw(" | "),
            ]
        })
        .collect::<Vec<Vec<Span>>>()
        .concat();
    let paragraph = Paragraph::new(Spans::from(binds))
        .block(Block::default().title("Keybindings").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}
