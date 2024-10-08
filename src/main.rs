#![feature(async_closure)]

use std::fs;

use meilisearch_sdk::{
    client::*,
    search::{SearchQuery, SearchResults},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Document {
    id: usize,
    name: String,
    content: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create a client (without sending any request so that can't fail)
    let meilisearch_url = option_env!("MEILISEARCH_URL").unwrap_or("http://localhost:7700");
    let meilisearch_api_key =
        option_env!("MEILISEARCH_API_KEY").unwrap_or("xWMHVnYwrZjr8IkkdrjmnpaFw10_w_78PlfcUOsnlB0");

    let client = Client::new(meilisearch_url, Some(meilisearch_api_key)).unwrap();

    let documents_index = client.index("documents");

    let doc1_filename = "doc1.md";
    let doc1 = fs::read_to_string(doc1_filename).expect("Should have been able to read the file");
    let doc = &Document {
        id: 1,
        name: doc1_filename.to_string(),
        content: doc1,
    };

    // Add some documents in the index. If the index 'documents' does not exist, Meilisearch creates it when you first add the documents.
    documents_index
        .add_documents(&[doc], Some("id"))
        .await
        .unwrap();

    // get some documents from the index
    let results: SearchResults<Document> = SearchQuery::new(&documents_index)
        .with_query("traverse")
        .build()
        .execute()
        .await
        .unwrap();

    println!("{:#?}", results);

    let retrieved_doc = &results.hits.get(0).unwrap().result;
    assert_eq!(doc.id, retrieved_doc.id);

    let content = &retrieved_doc.content;
    println!("{:#?}", content);

    // promopt llmaa model and read output
}
