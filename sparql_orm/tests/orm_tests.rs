fn get_update_headers() -> reqwest::HeaderMap {
    let mut headers = reqwest::HeaderMap::new();
}
fn write_to_sparql_db() -> reqwest::Result<_>{
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:7878/update");
    todo!()
}

#[test]
fn test_basic_insert_data() {
    assert!(false);
}
