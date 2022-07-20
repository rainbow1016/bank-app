use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;
use tonic::{transport::Server, Request, Response, Status};
use log::{info};

use user::user_server::{User, UserServer};
use user::{UserRequest, UserResponse};

pub mod user {
    tonic::include_proto!("users");
}


#[derive(Debug, Default)]
pub struct UserService {}

pub struct UserResponseConverter {
    id: String,
    name: String
}

impl From<UserResponseConverter> for UserResponse {
    fn from(data: UserResponseConverter) -> Self {
        UserResponse { user_id: data.id, username: data.name }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user(
        &self,
        request: Request<UserRequest>
    ) -> Result<Response<UserResponse>,  Status> {
        let req = request.into_inner();
        let user_id: i32 = req.user_id.parse::<i32>().unwrap();
        let pool = get_pool().await.unwrap();
        let result = sqlx::query_as!(
            UserResponse,
            r#"
            SELECT cast(id as varchar) as "user_id!", name as username
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&pool)
        .await.unwrap();

        Ok(Response::new(result))
    }
}

async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    Ok(
        PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@database:5432/bank")
        .await?
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let user_service = UserService::default();
    
    info!("Starting Users-service");
    Server::builder()
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
