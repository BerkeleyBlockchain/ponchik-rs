use ponchik::db;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use vercel_runtime::{
    http::bad_request, run, Body, Error,
    Request, RequestPayloadExt, Response, StatusCode,
};
use reqwest;
use std::collections::HashMap;

use tracing_subscriber;
use tracing::{event, span, Level, instrument};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    run(handler).await
}

#[derive(Debug, Deserialize, Serialize)]
struct Payload {
    payload: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[instrument]
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {

    let payload = req.payload::<Payload>();

    match payload {
        Ok(Some(payload)) => {
            match serde_json::from_str::<Value>(&payload.payload) {
                Ok(json_value) => {
                    event!(Level::DEBUG, "Received JSON: {:?}", json_value);
                    
                    if let Some(response_url) = json_value.get("response_url") {
                        println!("Response url: {}", response_url);
                        if let Some(actions) = json_value.get("actions") {
                            let action_val = actions[0].get("value").unwrap();
                            let user_id = &json_value["user"]["id"].as_str().unwrap();
                            let channel_id = &json_value["channel"]["id"].as_str().unwrap();
                            println!("ACTION VALUE: {}", action_val);
                            
                            let mut map = HashMap::new();
                            map.insert("replace_original", String::from("true"));

                            match action_val.as_str().unwrap() {
                                "mid_yes" => {
                                    map.insert("text", 
                                    format!("It's time for a midpoint checkin!\n\n *Did you get a chance to meet?*\n\n✅ <@{}> said that you've met!", user_id));
                                    

                                    let pool = db::db_init().await?;
                                    db::db_update_status(&pool, channel_id.to_string(), db::MeetingStatus::Closed(db::FinalStatus::Met)).await?;

                                    let _res = reqwest::Client::new().post(response_url.as_str().unwrap())
                                        .json(&map)
                                        .send()
                                        .await?;        
                                },
                                "mid_no" => {
                                    map.insert("text", 
                                    format!("It's time for a midpoint checkin!\n\n *Did you get a chance to meet?*\n\n*:C* <@{}> said that you have not scheduled yet.", user_id));
        
                                    // TODO: not sure what to make of this status yet

                                    let _res = reqwest::Client::new().post(response_url.as_str().unwrap())
                                        .json(&map)
                                        .send()
                                        .await?;        

                                },
                                "mid_scheduled" => {
                                    map.insert("text", 
                                    format!("It's time for a midpoint checkin!\n\n *Did you get a chance to meet?*\n\n📅 <@{}> said that your meeting is scheduled!", user_id));
        
                                    let pool = db::db_init().await?;
                                    db::db_update_status(&pool, channel_id.to_string(), db::MeetingStatus::Scheduled).await?;

                                    let _res = reqwest::Client::new().post(response_url.as_str().unwrap())
                                        .json(&map)
                                        .send()
                                        .await?;  

                                },
                                "close_yes" => {
                                    map.insert("text", 
                                    format!("Checking in! Did you guys get a chance to connect?\n\n🥳<@{}> said that you met! Great!", user_id));
        
                                    let pool = db::db_init().await?;
                                    db::db_update_status(&pool, channel_id.to_string(), db::MeetingStatus::Closed(db::FinalStatus::Met)).await?;

                                    let _res = reqwest::Client::new().post(response_url.as_str().unwrap())
                                        .json(&map)
                                        .send()
                                        .await?;  

                                },
                                "close_no" => {
                                    map.insert("text", 
                                    format!("Checking in! Did you guys get a chance to connect?\n\n😶‍🌫️<@{}> said no. Better luck next time!", user_id));
                                    
                                    // TODO: Check if it was previously scheduled or not
                                    let pool = db::db_init().await?;
                                    db::db_update_status(&pool, channel_id.to_string(), db::MeetingStatus::Closed(db::FinalStatus::Fail)).await?;

                                    let _res = reqwest::Client::new().post(response_url.as_str().unwrap())
                                        .json(&map)
                                        .send()
                                        .await?;  

                                },
                                _ => unreachable!()
                            }
                        }
                    } else {
                        bad_request(APIError {
                            message: "Invalid payload",
                            code: "invalid_payload",
                        })?;
                    }
                                        

                    Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(json_value.to_string().into())
                    .unwrap())
                },
                Err(_) => {
                    Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(serde_json::json!({"message": "could not parse JSON successfully"}).to_string().into())
                    .unwrap()) 
                }
            }
        }
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
    }

}