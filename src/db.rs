use reqwest::Client;
use serde_json::json;

const QUERY: &str = "
query ($id: Int, $page: Int, $perPage: Int, $search: String, $season: MediaSeason, $seasonYear: Int) {
  Page (page: $page, perPage: $perPage) {
      pageInfo {
          total
          currentPage
          lastPage
          hasNextPage
          perPage
      }
      media (id: $id, search: $search, season: $season, seasonYear: $seasonYear) {
          id
          season
          seasonYear
          title {
              romaji
          }
      }
  }
}
";


pub async fn test() -> serde_json::Value {
 let client = Client::new();

    let json = json!({"query": QUERY, "variables":{
      "page": 1,
      "perPage": 3,
      "season": "WINTER",
      "seasonYear": 2020
    }});

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