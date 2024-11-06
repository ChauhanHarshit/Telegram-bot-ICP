// // 1. IMPORT MANAGEMENT CANISTER
// use ic_cdk::api::management_canister::http_request::{
//     http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
//     TransformContext, TransformFunc,
// };

use candid::{CandidType, Nat};
use ic_cdk::api::*;
use ic_cdk_macros::*;
// use ic_cdk::api::management_canister::http_request::{ HttpHeader, HttpResponse};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};
use serde::Deserialize;
use serde_json::{json, Value}; // Import serde_json to parse JSON

// // Update method using the HTTPS outcalls feature
// #[ic_cdk::update]
// async fn get_icp_usd_exchange() -> String {
//     // 2. SETUP ARGUMENTS FOR HTTP GET request

//     // 2.1 Setup the URL
//     let host = "api.coingecko.com";
//     let url = format!(
//         "https://{}/api/v3/simple/price?ids=internet-computer&vs_currencies=usd",
//         host
//     );

//     // 2.2 Prepare headers for the system http_request call
//     let request_headers = vec![
//         HttpHeader {
//             name: "Host".to_string(),
//             value: host.to_string(),
//         },
//         HttpHeader {
//             name: "User-Agent".to_string(),
//             value: "exchange_rate_canister".to_string(),
//         },
//     ];

//     let request = CanisterHttpRequestArgument {
//         url: url.clone(),
//         method: HttpMethod::GET,
//         body: None,               // Not needed for GET request
//         max_response_bytes: None, // Optional
//         transform: Some(TransformContext {
//             function: TransformFunc(candid::Func {
//                 principal: ic_cdk::api::id(),
//                 method: "transform".to_string(),
//             }),
//             context: vec![],
//         }),
//         headers: request_headers,
//     };

//     // 3. MAKE HTTP REQUEST AND WAIT FOR RESPONSE

//     // Adjust the cycles based on your needs
//     let cycles = 50_000_000_000u64;

//     match http_request(request, cycles as u128).await {
//         // 4. DECODE AND RETURN THE RESPONSE
//         Ok((response,)) => {
//             // Convert the response body from Vec<u8> to String
//             let body_text = String::from_utf8(response.body)
//                 .expect("Response body is not valid UTF-8.");

//             // Parse the JSON response to extract the exchange rate
//             let json: Value = serde_json::from_str(&body_text)
//                 .expect("Failed to parse JSON response.");

//             // Extract the exchange rate
//             if let Some(rate) = json["internet-computer"]["usd"].as_f64() {
//                 format!("ICP-USD Exchange Rate: ${}", rate)
//             } else {
//                 "Failed to extract exchange rate from response.".to_string()
//             }
//         }
//         Err((r, m)) => {
//             let message =
//                 format!("The http_request resulted in an error. RejectionCode: {r:?}, Error: {m}");
//             message
//         }
//     }
// }

// // Transform function to process the HTTP response
// #[ic_cdk::query]
// fn transform(raw: TransformArgs) -> HttpResponse {
//     // You can modify headers or the body here if needed
//     HttpResponse {
//         status: raw.response.status.clone(),
//         body: raw.response.body.clone(),
//         headers: vec![], // Include any necessary headers
//     }
// }

// 1. IMPORT MANAGEMENT CANISTER
// use ic_cdk::api::management_canister::http_request::{
//     http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
//     TransformContext, TransformFunc,
// };
// use serde_json::json; // Import for creating JSON payloads
// use serde_json::Value; // Import for parsing JSON responses

// #[ic_cdk::update]
// async fn get_openai_completion(prompt: String) -> String {
//     // Set the OpenAI API endpoint and host
//     let host = "api.openai.com";
//     let url = format!("https://{}/v1/completions", host);

//     // Prepare headers, including the Authorization header with your OpenAI API key
//     let api_key = ""; //replace this with api key
//     let request_headers = vec![
//         HttpHeader {
//             name: "Host".to_string(),
//             value: host.to_string(),
//         },
//         HttpHeader {
//             name: "Authorization".to_string(),
//             value: format!("Bearer {}", api_key),
//         },
//         HttpHeader {
//             name: "Content-Type".to_string(),
//             value: "application/json".to_string(),
//         },
//     ];

//     // Construct the JSON payload for the OpenAI completion request
//     let payload = json!({
//         "model": "gpt-3.5-turbo",
//         "prompt": prompt,
//         "max_tokens": 500
//     });
//     let request_body = serde_json::to_vec(&payload).expect("Failed to serialize payload");

//     // Set up the HTTP request argument
//     let request = CanisterHttpRequestArgument {
//         url: url.clone(),
//         method: HttpMethod::POST,
//         body: Some(request_body),
//         max_response_bytes: None, // Optional
//         transform: Some(TransformContext {
//             function: TransformFunc(candid::Func {
//                 principal: ic_cdk::api::id(),
//                 method: "transform".to_string(),
//             }),
//             context: vec![],
//         }),
//         headers: request_headers,
//     };

//     // Specify the cycles for the HTTP outcall
//     let cycles = 100_000_000_000u64;

//     // Make the HTTP request and process the response
//     match http_request(request, cycles as u128).await {
//         Ok((response,)) => {
//             let body_text = String::from_utf8(response.body)
//                 .expect("Response body is not valid UTF-8.");
//             ic_cdk::api::print(format!("Raw response: {}", body_text)); // Log the response

//             // Parse the JSON response to extract the completion text
//             let json: Value = serde_json::from_str(&body_text)
//                 .expect("Failed to parse JSON response.");

//             if let Some(choice) = json["choices"][0]["text"].as_str() {
//                 choice.to_string()
//             } else {
//                 "Failed to extract text completion from response.".to_string()
//             }
//         }
//         Err((r, m)) => {
//             format!("The http_request resulted in an error. RejectionCode: {r:?}, Error: {m}")
//         }
//     }

// }

// // Transform function to process the HTTP response
// #[ic_cdk::query]
// fn transform(raw: TransformArgs) -> HttpResponse {
//     HttpResponse {
//         status: raw.response.status.clone(),
//         body: raw.response.body.clone(),
//         headers: vec![], // You may include any additional necessary headers here
//     }
// }

#[ic_cdk::update]
async fn send_telegram_message(message: String) -> String {
    // Set the Telegram API endpoint and bot token
    let bot_token = "BOT_TOKEN"; // Replace this with your actual Telegram bot token
    let chat_id = "CHAT_ID"; // Replace this with the chat ID you want to send messages to
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    // Prepare headers (Telegram API does not require authorization headers for bot requests)
    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    // Construct the JSON payload for the Telegram message
    let payload = json!({
        "chat_id": chat_id,
        "text": message
    });
    let request_body = serde_json::to_vec(&payload).expect("Failed to serialize payload");

    // Set up the HTTP request argument
    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method: HttpMethod::POST,
        body: Some(request_body),
        max_response_bytes: None, // Optional
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: request_headers,
    };

    // Specify the cycles for the HTTP outcall
    let cycles = 100_000_000_000u64;

    // Make the HTTP request and process the response
    match http_request(request, cycles as u128).await {
        Ok((response,)) => {
            let body_text =
                String::from_utf8(response.body).expect("Response body is not valid UTF-8.");

            ic_cdk::api::print(format!("Raw response from Telegram: {}", body_text)); // Log the response

            let json: Value =
                serde_json::from_str(&body_text).expect("Failed to parse JSON response.");

            if let Some(ok) = json["ok"].as_bool() {
                if ok {
                    "Message sent successfully!".to_string()
                } else {
                    "Failed to send message.".to_string()
                }
            } else {
                "Unexpected response structure.".to_string()
            }
        }
        Err((r, m)) => {
            format!("The http_request resulted in an error. RejectionCode: {r:?}, Error: {m}")
        }
    }
}

// Transform function to process the HTTP response
#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: vec![], // You may include any additional necessary headers here
    }
}

// Define your request and response structures
#[derive(CandidType, Deserialize)]
pub struct TelegramMessage {
    update_id: i64,
    message: TelegramMessageContent,
}

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

// #[derive(CandidType)]
// pub struct HttpResponse {
//     pub status: u16,
//     pub headers: Vec<HeaderField>,
//     pub body: Vec<u8>,
// }

#[derive(CandidType, Deserialize)]
pub struct TelegramMessageContent {
    message_id: i64,
    from: TelegramUser,
    chat: TelegramChat,
    text: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct TelegramUser {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct TelegramChat {
    id: i64,
    chat_type: String,
}

// Define HttpRequest if it is not available in any library
#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

// Handle incoming messages from Telegram
#[query]
async fn receive_telegram_message(req: HttpRequest) -> HttpResponse {
    // Log the incoming request for debugging
    ic_cdk::print(format!(
        "Received a request with method: {} and URL: {}",
        req.method, req.url
    ));

    // Attempt to deserialize the message and log the outcome
    let message: TelegramMessage = match serde_json::from_slice(&req.body) {
        Ok(msg) => {
            ic_cdk::print("Successfully deserialized Telegram message.");
            msg
        }
        Err(e) => {
            ic_cdk::print(format!("Failed to deserialize Telegram message: {:?}", e));
            return HttpResponse {
                status: Nat::from(400 as u64), // Return a 400 Bad Request on deserialization failure
                headers: vec![HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                }],
                body: b"{\"error\": \"Bad Request: Failed to parse message\"}".to_vec(),
            };
        }
    };

    // Process the deserialized message and log its content
    if let Some(text) = message.message.text {
        let response = format!("Received message text: {}", text);
        ic_cdk::print(response.clone());
    } else {
        ic_cdk::print("No message text found in Telegram message.");
    }

    // Log response before returning
    ic_cdk::print("Sending a success response to Telegram.");

    // Respond with a success message
    HttpResponse {
        status: Nat::from(200 as u64), // Status OK
        headers: vec![HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }],
        body: b"{\"status\": \"ok\"}".to_vec(),
    }
}


