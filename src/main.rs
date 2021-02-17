use crossterm::event::Event;
use std::time::{Duration, SystemTime};

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
    //println!("{:#?}", anilist::test().await.get("data").unwrap().get("Page").unwrap());
    println!(
        "{:?}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    //dbg!(&test);

    let mut app = app::App::new("Lista anime".to_owned());
    app.authorize();

    let test: app::RecievedData = serde_json::from_value(anilist::test(&app).await).unwrap();

    let dummy_list = app::StatefulList::with_items(test.data.unwrap().page.unwrap().media);
    app.animes = dummy_list;

    let mut terminal = terminal::create_terminal();
    terminal::draw_frame(&mut terminal, &mut app);
    let event = terminal::events_test();

    while !app.should_exit {
        let ev = event.recv().unwrap();
        app.handle_input(ev).await;
        terminal::draw_frame(&mut terminal, &mut app);
    }
    terminal::leave_terminal();
}
