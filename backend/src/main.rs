use log::info;
use tide::Request;

pub mod users;
use users::user_client::UserClient;
use users::UserRequest;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();
    info!("Starting Tide Server");
    let mut app = tide::new();
    app.at("/user/:id").get(get_user);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn get_user(req: Request<()>) -> tide::Result {
    let id = req.param("id")?;
    info!("Retrieving user data for user id: {}", &id);

    let mut client = UserClient::connect("http://users-service:50051").await?;
    let user_data = client
        .get_user(tonic::Request::new(UserRequest {
            user_id: id.to_string(),
        }))
        .await
        .unwrap()
        .into_inner();

    let mut resp = tide::Response::new(200);
    let json_data = serde_json::to_value(&user_data).unwrap();
    resp.set_body(json_data);
    Ok(resp)
}
