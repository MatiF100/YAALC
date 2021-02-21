use crate::{
    app::{Anime, App, PagedAnime, RecievedData, RecievedPage, StatefulList},
    terminal,
};
use reqwest::Client;
use serde_json::json;

pub mod auth;
mod filters;
mod queries;

//Test function that sends first request
//Used mainly for testing purposes
pub async fn test(app: &App) -> serde_json::Value {
    let client = Client::new();
    //auth::auth();

    /*
    let data: filters::TestQuery = filters::TestQuery{
        page: Some(1),
        perPage: None,
        season: Some("WINTER".to_owned()),
        seasonYear: Some(2020)
    };
    */

    /*
    let json = json!({"query": QUERY, "variables":{
      "page": 1,
      "season": "WINTER",
      "seasonYear": 2020
    }});
    */

    let mut data = filters::Variables::new();
    data.page_setup(1, 50);
    data.season_setup("WINTER".to_owned(), 2021);
    data.set_anime_type();

    let json = json!({"query": queries::TEST_QUERY, "variables": &data});
    println!("{:?}", json!(&data));

    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", app.get_token().unwrap())
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&resp).unwrap()

    //let result: serde_json::Value = serde_json::from_str(&resp).unwrap();
}

//Function sending request for animes filtered only by name
pub async fn search_anime_by_name(search: String) -> Vec<Anime> {
    let client = Client::new();
    let mut query_args = filters::Variables::new();
    let mut page_index = 1;
    query_args.set_anime_type();
    query_args.search_setup(search);
    let mut output: Vec<Anime> = Vec::new();

    loop {
        query_args.page_setup(page_index, 50);
        let query = json!({"query": queries::TEST_QUERY, "variables": &query_args});
        let response = client
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(query.to_string())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let animes: RecievedData = serde_json::from_str(&response).unwrap();

        //let tmp = animes.data.unwrap().page.unwrap().media.unwrap();

        let mut page_response = get_page_from_recieved_data(animes);
        output.append(&mut page_response.media);

        if page_response.page_info.has_next_page.unwrap() {
            page_index += 1;
        } else {
            break;
        }
        //println!("Dupa: {}, {}, {}",page_index,page_response.page_info.current_page.unwrap(), page_response.page_info.last_page.unwrap());
    }
    output
    //tmp
}

//Function unpacking data recieved from API endpoint, and checking for errors
fn get_page_from_recieved_data(data: RecievedData) -> PagedAnime {
    match data {
        RecievedData {
            data: Some(data),
            errors: None,
        } => match data {
            RecievedPage { page: Some(page) } => page,
            RecievedPage { page: None } => {
                panic!("Recieved data does not contain Page field! Aborting!")
            }
        },
        RecievedData {
            data: Some(_),
            errors: Some(err),
        } => panic!("Recieved data, but unexpected error occured: {:?}", err),
        RecievedData {
            data: None,
            errors: Some(err),
        } => panic!("No data recieved! Unexpected error occured: {:?}", err),
        _ => panic!("No data, nor errors recieved!"),
    }
}
