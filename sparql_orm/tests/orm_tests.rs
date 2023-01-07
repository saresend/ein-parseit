use sparql_orm::query_build::{QueryFragment, SparqlQuery, run_sparql_generation};
use reqwest;
use reqwest::blocking::Response;

fn get_update_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::CONTENT_TYPE, "application/sparql-update".parse().unwrap());
    headers
}
fn write_to_sparql_db<T: SparqlQuery + QueryFragment>(val: T) -> reqwest::Result<Response>{
    let client = reqwest::blocking::Client::new();
    let body_content = run_sparql_generation(val);   
    client.post("http://localhost:7878/update").headers(get_update_headers()).body(body_content).send()
}

#[test]
fn test_basic_insert_data() {
    assert!(false);
}
