use crossterm::event::Event;


mod anilist;
mod app;
mod terminal;
const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

#[tokio::main]
async fn main() {
    println!("{:#?}", anilist::test().await);

    let mut app = app::App::new("Lista anime".to_owned());
    let dummy_list = app::StatefulList::with_items(
        TASKS
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>(),
    );
    app.animes = dummy_list;

    let mut terminal = terminal::create_terminal();
    terminal::draw_frame(&mut terminal, &mut app);
    let event = terminal::events_test(&mut terminal);

    loop {
        let ev = event.recv().unwrap();
        match ev {
            Event::Key(key) => match key {
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Char('q'),
                    modifiers: _,
                } => break,
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Up,
                    modifiers: _,
                } => app.animes.previous(),
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Down,
                    modifiers: _,
                } => app.animes.next(),
                _ => println!("Pressed: {:?}", key),
            },
            //Event::Resize(_, _) => terminal::draw_frame(&mut terminal, &mut app),
            _ => (),
        }
        terminal::draw_frame(&mut terminal, &mut app);
    }
}
