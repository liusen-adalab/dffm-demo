use anyhow::Result;
use dffm_demo::{
    models::{db_conn, users::read_by_id},
    msg::Requst,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    println!("running..");
    loop {
        responder.recv(&mut msg, 0).unwrap();
        let msg = msg.as_str().unwrap();
        println!("Received {}", msg);

        let req: Requst = serde_json::from_str(msg).unwrap();
        let resp = handle_req(req).await.unwrap();
        responder.send(&resp, 0).unwrap();
    }
}

async fn handle_req(req: Requst) -> Result<String> {
    match req {
        Requst::GetUserById(id) => {
            let mut conn = db_conn().await;
            let user = read_by_id(&mut conn, id).await?;
            let user_str = serde_json::to_string(&user)?;
            Ok(user_str)
        }
    }
}
