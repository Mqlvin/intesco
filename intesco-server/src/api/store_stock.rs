use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::{Value::{self}, json};


#[derive(Serialize, Deserialize)]
pub struct StockLookupParams {
    store_uuid: String,
    query: String,
    max_results: Option<u16>,
    page: Option<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct NameLookupParams {
    tpnc: String,
}


pub async fn get_stock(params: StockLookupParams) -> Result<Value, String> {
    let client = Client::builder().build().map_err(|e| format!("Failed to initialise backend client: {}", e).to_string())?;
    let url = "https://xapi.tesco.com/";

    let json_data = json!([{
        "query": SEARCH_QUERY,
        "operationName": "SearchProduct",
        "variables": {
            "query": params.query,
            "locationUuid": params.store_uuid,
            "page": params.page.unwrap_or(1),
            "count": params.max_results.unwrap_or(20),
            "config": {
                "variant": "default"
            },
            "storeProductOptions": {
                "page": 1,
                "count": 10, // change to get more nearby stores with product, ill just show one
                "conditions": ["SHOW_WHEN_UNAVAILABLE"],
            },
            "includeProductAvailability": true,
            "includeProductLocation": true,
            "channel": "INSTORE"
        },
        "extensions": {
            "mfeName": "mfe-header"
        }
    }]);

    let response = client.post(url).json(&json_data)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip, deflate, zstd")
        .header("Connection", "keep-alive")
        .header("Language", "en-GB")
        .header("Region", "UK")
        .header("x-apikey", "TvOSZJHlEk0pjniDGQFAc9Q59WGAR4dA")
        .send().await
        .map_err(|err| format!("Request failed: {}", err).to_string())?;

    let mut body: Value = response.json().await
        .map_err(|err| format!("Failed to read response body: {}", err).to_string())?;

    let array = body.as_array_mut().ok_or_else(|| "Bad response, not singleton array")?;
    let singleton_array = array.get_mut(0).ok_or_else(|| "Bad response, array was empty")?
        .as_object_mut().ok_or_else(|| "Bad response, singleton item should be JSON object")?;
    let data_obj = singleton_array.get_mut("data").ok_or_else(|| "Bad response, no data object")?;
    let search_result_obj = data_obj.get_mut("search").ok_or_else(|| "Bad response, no search results")?
        .as_object_mut().ok_or_else(|| "Bad response, search result wasn't object")?;
    let results_arr = search_result_obj.get_mut("results").ok_or_else(|| "Bad response, couldn't get results array")?
        .as_array_mut().ok_or_else(|| "Bad response, results wasn't an array")?;

    let clean_results: Vec<Value> = results_arr
        .iter()
        .map(|res| {
            let mut obj = res.clone();
            if let Value::Object(ref mut outer) = obj {
                if let Some(Value::Object(node)) = outer.remove("node") {
                    for (k, v) in node {
                        outer.insert(k, v);
                    }
                }
                outer.remove("gtin");
                outer.remove("details");
            }
            obj
        })
        .collect();

    search_result_obj.insert("results".to_string(), Value::Array(clean_results));

    Ok(data_obj.clone())
}

pub async fn get_item_name(params: NameLookupParams) -> Result<Value, String> {
    let client = Client::builder().build().map_err(|e| format!("Failed to initialise backend client: {}", e).to_string())?;
    let url = "https://xapi.tesco.com/";

    let json_data = json!([{
        "query": ITEM_NAME_QUERY,
        "operationName": "GetProductMultiseller",
        "variables": {
            "configs": {},
            "tpnc": params.tpnc,
        }
    }]);

    let response = client.post(url).json(&json_data)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip, deflate, zstd")
        .header("Connection", "keep-alive")
        .header("Language", "en-GB")
        .header("Region", "UK")
        .header("x-apikey", "TvOSZJHlEk0pjniDGQFAc9Q59WGAR4dA")
        .send().await
        .map_err(|err| format!("Request failed: {}", err).to_string())?;

    let mut body: Value = response.json().await
        .map_err(|err| format!("Failed to read response body: {}", err).to_string())?;

    let array = body.as_array_mut().ok_or_else(|| "Bad response, not singleton array")?;
    let singleton_array = array.get_mut(0).ok_or_else(|| "Bad response, array was empty")?
        .as_object_mut().ok_or_else(|| "Bad response, singleton item should be JSON object")?;
    let data_obj = singleton_array.get_mut("data").ok_or_else(|| "Bad response, no data object")?;
    let product_obj = data_obj.get("product").ok_or_else(|| "Bad response, no product object")?;
    let title = product_obj.get("title").ok_or_else(|| "Bad response, no product object")?
        .as_str().ok_or_else(|| "Bad response, title was not a string")?;

    Ok(Value::String(title.to_string()))
}

const SEARCH_QUERY: &str= "
    query SearchProduct(
        $query: String!
        $locationUuid: ID
        $page: Int
        $count: Int
        $config: BrowseSearchConfig
        $storeProductOptions: storeProductStockArgType
        $includeProductAvailability: Boolean = false
        $includeProductLocation: Boolean = false
        $channel: ChannelType
    ) {
        search(
            query: $query
            locationUuid: $locationUuid
            page: $page
            count: $count
            config: $config
            channel: $channel
        ) {
            pageInformation: info {
                ...PageInformation
            }
            results {
                node {
                    ...ProductItem
                }
            }
        }
    }
    fragment PageInformation on ListInfoInterface {
        totalCount: total
        pageNo: page
        count
        matchType
        query {
            searchTerm
        }
    }
    fragment ProductItem on ProductInterface {
        title
        id
        defaultImageUrl
        productLocation @include(if: $includeProductLocation) {
            ... on StoreProductLocationType {
                aisleOrientation
                aisle
                moduleNumber
                shelfNumber
                productPosition
            }
        }
        availability @include(if: $includeProductAvailability) {
            ... on SelectedStoreProductAvailabilityType {
                status
                quantity
                location(storeProductOptions: $storeProductOptions) {
                    stores {
                        storeInformation {
                            ... Stores
                        }
                        availability {
                            status
                            quantity
                        }
                    }
                }
            }
        }
        price {
            actual
            unitPrice
            unitOfMeasure
        }
        details {
            components {
                ... on AdditionalInfo {
                    __typename
                    isLowEverydayPricing
                    isLowPricePromise
                }
            }
        }
        promotions {
            promotionId: id
            promotionType
            startDate
            endDate
            offerText: description
            promotionAttributes: attributes
            price {
                beforeDiscount
                afterDiscount
            }
            unitSellingInfo
        }
    }
    fragment Stores on StoreLocationType {
        locationUuid
        distance
        distanceUnit
        latitude
        longitude
        address {
            name
        }
        openingHours {
            mo {
                ...StoreHours
            }
            tu {
                ...StoreHours
            }
            we {
                ... StoreHours
            }
            th {
                ... StoreHours
            }
            fr {
                ... StoreHours
            }
            sa {
                ... StoreHours
            }
            su {
                ...StoreHours
            }
        }
    }
    fragment StoreHours on HoursForDayType {
        isOpen
        open
        close
    }
";

const ITEM_NAME_QUERY: &str = "
    query GetProductMultiseller(
        $tpnc: String,
        $configs: [ConfigArgType],
    ) { 
        product(tpnc: $tpnc, configs: $configs) {
        id    title     
    }
}
";
