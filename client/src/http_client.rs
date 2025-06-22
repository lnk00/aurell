use aurell_types::ApiResponse;

const BASE_URL: &str = "http://localhost:3000/api";

pub async fn post<T, K>(url: &str, body: &T) -> ApiResponse<K>
where
    T: serde::Serialize,
    K: serde::Serialize + serde::de::DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}{}", BASE_URL, url))
        .json(body)
        .send()
        .await
        .unwrap()
        .json::<ApiResponse<K>>()
        .await
        .unwrap();

    response
}
