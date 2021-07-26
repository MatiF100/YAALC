//Read queries
pub const TEST_QUERY: &str = "
query ($id: Int, $page: Int, $perPage: Int, $search: String, $season: MediaSeason, $seasonYear: Int, $type: MediaType) {
  Page (page: $page, perPage: $perPage) {
      pageInfo {
          total
          currentPage
          lastPage
          hasNextPage
          perPage
      }
      media (id: $id, search: $search, season: $season, seasonYear: $seasonYear, type: $type) {
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

pub const CURRENT_USER_DATA: &str = "
query {
    Viewer{
        id,
        name,
        about,
        statistics{
            anime{
                count,
                meanScore,
                standardDeviation,
                minutesWatched,
                episodesWatched,
                chaptersRead,
                volumesRead
            }
            manga{
                count,
                meanScore,
                standardDeviation,
                minutesWatched,
                episodesWatched,
                chaptersRead,
                volumesRead
            }
        }
    }
}
";

//Mutations (Edition queries)
