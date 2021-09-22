use actix_web::{middleware::Logger, error, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize, Deserializer};
use log::{debug, error, log_enabled, info, Level};
use std::collections::HashMap;

// https://github.com/serde-rs/json
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Info2 {
    username: String,
    // 这样也可以，如果都指定了，会按下面这个为准
    // #[serde(rename = "y")]
    my_addr: String,
    #[serde(default)]
    age: i32,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String
}

#[post("/idx2")]
async fn index2(info: web::Json<Info2>) -> impl Responder {
    println!("Welcome {} addr: {} age: {}!", info.username, info.my_addr, info.age);
    let mut vec:Vec<Country> = Vec::new();
    vec.push(Country{country_code: "PH".to_string(), country_name: "Philippines".to_string()});
    vec.push(Country{country_code: "MY".to_string(), country_name: "Malaysia".to_string()});
    vec.push(Country{country_code: "ID".to_string(), country_name: "Indonesia".to_string()});
    let mut m :HashMap<String, Vec<Country>> = HashMap::new();
    m.insert("data".to_string(), vec);
   // let m = ("body", vec);
    return web::Json(m);
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let person = Info2 {
        username: "a".to_string(),
        my_addr: "add".to_string(),
        age: 2
    };
    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("{}", json);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();


    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .wrap(Logger::default())
            //.wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(echo)
            .service(index)
            .service(index2)
            .app_data(json_config)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8880")?
        .run()
        .await
}

/*
curl -i --request POST 'http://127.0.0.1:8880/idx2' \
--header 'Content-Type: application/json' \
--data-raw '{
"username": "a",
"myAddr": "add"
}'
// 如果传的参数不匹配会 409
You can use Option<T> as the type for a field to have it be optional. If a field is missing during deserialization, the field is set to None, otherwise it is set to Some(value).

#[derive(Deserialize)]
struct IncomingPayload {
    pub field1: Option<i32>,
    pub field2: Option<String>,
}
For types that implement Default, you can also use the #[serde(default)] to set the field to the default value if the field is missing.

#[derive(Deserialize)]
struct IncomingPayload {
    #[serde(default)] // default = 0
    pub field1: i32,
    #[serde(default)] // default = empty string
    pub field2: String,
}
 */