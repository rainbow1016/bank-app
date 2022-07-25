use users::user_client::UserClient;
use users::UserRequest;

pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(UserRequest {
        user_id: "1".to_string(),
    });

    let response = client.get_user(request).await?;

    println!("Response={:?}", response);
    Ok(())
}
