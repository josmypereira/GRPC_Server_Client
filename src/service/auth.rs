use auth_service::auth_service_server::{AuthService, AuthServiceServer};
use auth_service::{SignInReq, SignInResp, SignOutReq, SignOutResp};
use tonic::{Request, Response, Status};


pub mod auth_service{
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl AuthService for AuthenticationService {
    async fn sign_in(&self, request: Request<SignInReq>) -> Result<Response<SignInResp>, Status>{
        let user: SignInReq = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply: SignInResp = SignInResp{
            user: Some(auth_service::User {
                first_name: String::from("josmy"), 
                last_name: String::from("pereira"), 
                email: String::from("josmy")
            }),
            token: String::from("josmy"),
            refresh_token: String::from("josmy"),
        };
        Ok(Response::new(reply))
    }

    async fn sign_out(&self, request: Request<SignOutReq>) -> Result<Response<SignOutResp>, Status>{
        let user: SignOutReq = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply: SignOutResp = SignOutResp{};
        Ok(Response::new(reply))
    }

    async fn sign_up(&self, request: Request<auth_service::User>) -> Result<Response<auth_service::User>, Status> {
        let user: auth_service::User = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = auth_service::User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        Ok(Response::new(reply))
    }
    
    async fn update(&self, request: Request<auth_service::User>) -> Result<Response<auth_service::User>, Status> {
        let user: auth_service::User = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = auth_service::User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        Ok(Response::new(reply))
    }
    

}

pub fn authentication_service() -> AuthServiceServer<AuthenticationService> {
    let auth = AuthenticationService::default();
    AuthServiceServer::new(auth)
}