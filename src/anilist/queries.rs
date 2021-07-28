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

pub const GET_ANIME_LIST: &str = "
query($userId: Int, $type: MediaType){
    MediaListCollection(userId: $userId, type: $type){
        lists{
            entries{
                id,
                userId,
                mediaId,
                status,
                score,
                progress,
                media{
                    id,
                    idMal,
                    title{
                        native,
                        romaji,
                        english
                    },
                    season,
                    seasonYear,
                    episodes,
                    genres,
                    status,
                    duration
                }
            },
            name,
            isCustomList,
            isSplitCompletedList,
            status
        },
        hasNextChunk
    }
}
";

//Mutations (Edition queries)
