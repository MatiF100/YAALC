use crate::app::Anime;
use reqwest::Client;
use serde_json::json;

mod filters;
mod queries;

pub async fn test() -> serde_json::Value {
    let client = Client::new();

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
    data.page_setup(1, 5);
    data.season_setup("WINTER".to_owned(), 2021);

    let json = json!({"query": queries::TEST_QUERY, "variables": &data});
    println!("{:?}", json!(&data));

    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
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

pub async fn search_anime_by_name() -> Vec<Anime> {
    Vec::new()
}
