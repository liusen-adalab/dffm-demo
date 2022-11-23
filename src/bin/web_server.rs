use std::str::FromStr;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dffm_demo::msg::Requst;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, metadata::LevelFilter};
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Layer};

// curl --request GET 'http://localhost:8889/api/health'
async fn health() -> impl Responder {
    debug!("health check");
    HttpResponse::Ok().body("server alive\n")
}

// curl --request GET 'http://localhost:8889/api/user_profile/1'
#[get("/user_profile/{user_id}")]
async fn user_profile(path: web::Path<i32>) -> String {
    debug!("user profile");
    let id = path.into_inner();
    let resp = query_db(Requst::GetUserById(id)).await.unwrap();
    resp
}

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
    tel_number: String,
}

#[post("/register")]
async fn register2(_info: web::Json<Info>) -> String {
    todo!()
}

async fn query_db(req: Requst) -> Result<String> {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    let req = serde_json::to_vec(&req).unwrap();
    requester.send(req, 0).unwrap();
    requester.recv(&mut msg, 0).unwrap();
    let msg = msg.as_str().unwrap().to_string();
    debug!("Received db resp: {}", msg);
    Ok(msg)
}

fn init_logger() {
    let level = LevelFilter::from_str("debug").unwrap();
    let std_out = {
        let filter = EnvFilter::from_default_env().add_directive(level.into());
        fmt::Layer::new()
            .with_writer(std::io::stdout)
            .with_filter(filter)
    };
    let collector = tracing_subscriber::registry().with(std_out);
    tracing::subscriber::set_global_default(collector).expect("failed to init logger");
}

#[actix_web::main]
async fn main() -> Result<()> {
    init_logger();
    info!("starting server");
    let bind = ("0.0.0.0", 8889);
    let server = HttpServer::new(move || {
        let api_scope = web::scope("/api");
        let api_scope = api_scope.service(web::resource("/health").guard(guard::Get()).to(health));
        let api_scope = api_scope.service(user_profile);

        App::new().service(api_scope)
    })
    .bind(bind)?
    .run();

    server.await?;
    Ok(())
}
