use reqwest::Client;
use serde_json::Value;

pub async fn get_nearby_stores(long: f64, lat: f64) -> Result<Value, String> {
    let client = match Client::builder().user_agent("Mozilla/4.0 (Windows; Linus x64_80) Gecko/201001 Firefox/150.0").build() {
        Ok(val) => val,
        Err(err) => { return Err(err.to_string()) }
    };

    let response = match client.get("https://www.tesco.com/store-locator/searchapi")
        .header("Connection", "keep-alive")
        .header("Accept-Encoding", "gzip, deflate, zstd")
        .header("Accept", "application/json")
        .query(&[("q", format!("{},{}", lat, long)),("l", "en".to_string()),])
        .send()
        .await {
            Ok(val) => val,
            Err(err) => { return Err(err.to_string()) }
        };

    let body: Value = response
        .json()
        .await
        .map_err(|error| error.to_string())?;


    match body.get("response") {
        Some(val) => { 
            let mut temp = val.clone();
            clean_store_listings(&mut temp);
            Ok(temp)
        },
        None => { Err("No stores in response".to_string()) }
    }
}

fn clean_store_listings(stores: &mut Value) -> () {
    if let Some(entities) = stores.get_mut("entities") {
        entities.as_array_mut().expect("entities was an array").iter_mut().for_each(|store| {
            if let Some(store_obj) = store.as_object_mut() {
                store_obj.remove("url");
                store_obj.remove("derivedData");
                store_obj.remove("listings");
                store_obj.insert("name".to_string(), (store_obj.get("profile").expect("has profile").get("c_bRANCH_NAME").expect("branch has name")).clone());
                store_obj.insert("address".to_string(), (store_obj.get("profile").expect("has profile").get("address").expect("branch has address")).clone());
                store_obj.remove("profile");
            }
        });
    }
}
