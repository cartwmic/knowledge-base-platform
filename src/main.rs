use meilisearch_sdk::{client::*, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}

fn main() {
    // Create a client (without sending any request so that can't fail)
    let MEILISEARCH_URL = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    let MEILISEARCH_API_KEY =
        option_env!("MEILISEARCH_API_KEY").unwrap_or("xWMHVnYwrZjr8IkkdrjmnpaFw10_w_78PlfcUOsnlB0");

    let client = Client::new(MEILISEARCH_URL, Some(MEILISEARCH_API_KEY)).unwrap();

    // let documents = client
    //     .get_index("documents")
    //     .await
    //     .map_err(|err: Error| match err {
    //         _ => err,
    //     });
    //
    // // Create a new index called movies and access it
    // let task = client.create_index("movies", None).await.unwrap();
    //
    // // Wait for the task to complete
    // let task = task.wait_for_completion(&client, None, None).await.unwrap();
    //
    // // Try to get the inner index if the task succeeded
    // let index = task.try_make_index(&client).unwrap();
    //
    // assert_eq!(index.as_ref(), "movies");
    //
    // let movies = client.index("movies");
    //
    // // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
    // movies
    //     .add_documents(
    //         &[
    //             Movie {
    //                 id: 1,
    //                 title: String::from("Carol"),
    //                 genres: vec!["Romance".to_string(), "Drama".to_string()],
    //             },
    //             Movie {
    //                 id: 2,
    //                 title: String::from("Wonder Woman"),
    //                 genres: vec!["Action".to_string(), "Adventure".to_string()],
    //             },
    //             Movie {
    //                 id: 3,
    //                 title: String::from("Life of Pi"),
    //                 genres: vec!["Adventure".to_string(), "Drama".to_string()],
    //             },
    //             Movie {
    //                 id: 4,
    //                 title: String::from("Mad Max"),
    //                 genres: vec!["Adventure".to_string(), "Science Fiction".to_string()],
    //             },
    //             Movie {
    //                 id: 5,
    //                 title: String::from("Moana"),
    //                 genres: vec!["Fantasy".to_string(), "Action".to_string()],
    //             },
    //             Movie {
    //                 id: 6,
    //                 title: String::from("Philadelphia"),
    //                 genres: vec!["Drama".to_string()],
    //             },
    //         ],
    //         Some("id"),
    //     )
    //     .await
    //     .unwrap();
}
