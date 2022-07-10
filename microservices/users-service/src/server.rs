use tonic::{transport::Server, Request, Response, Status};

use user::user_server::{User, UserServer};
use user::{UserRequest, UserResponse};

pub mod user {
    tonic::include_proto!("users");
}


#[derive(Debug, Default)]
pub struct UserService {}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<UserRequest>
    ) -> Result<Response<UserResponse>,  Status> {
        let req = request.into_inner();
        Ok(Response::new(UserResponse{
            user_id: req.user_id,
            username: "test username".to_string()
        }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let user_service = UserService::default();
    

    Server::builder()
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
