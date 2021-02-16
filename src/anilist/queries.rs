pub const TEST_QUERY: &str = "
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
          idMal
          season
          seasonYear
          episodes
          genres
          status
          duration
          type
          title {
              native
              romaji
              english
          }
      }
  }
}
";
