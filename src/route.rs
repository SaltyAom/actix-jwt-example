pub mod api {
    use actix_web::{ HttpResponse, get, post, web, cookie, HttpRequest, HttpMessage };
    use serde::{ Serialize, Deserialize };

    use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
    use jsonwebtoken::errors::ErrorKind;

    use std::env;

    use crate::libs::{ get_time };

    #[derive(Serialize, Deserialize)]
    pub struct AuthPayload {
        sub: String,
        name: String,
        exp: u128
    }

    #[derive(Serialize, Deserialize)]
    pub struct Login {
        name: String,
        password: String
    }

    #[post("/login")]
    pub async fn login(body: web::Form<Login>) -> HttpResponse {
        let auth_payload = AuthPayload {
            sub: "authToken".to_owned(),
            name: body.name.to_owned(),
            exp: get_time()
        };

        let auth_token = match encode(
            &Header::default(), 
            &auth_payload, 
            &EncodingKey::from_secret(&env::var("jwt_secret").unwrap().as_ref())
        ) {
            Ok(value) => value,
            Err(_) => panic!()
        };

        let auth_cookie = cookie::Cookie::build("authToken", auth_token.to_owned())
            .path("/")
            .http_only(true)
            .finish();

        HttpResponse::Ok()
            .cookie(auth_cookie)
            .body(
                format!("{{
                    \"success\": true,
                    \"token\": \"{}\"
                }}", auth_token)
            )
    }

    #[post("/api/get_profile")]
    pub async fn get_profile(req: HttpRequest) -> HttpResponse {
        if req.cookie("authToken").is_none() {
            return HttpResponse::Ok()
                .content_type("application/json")
                .body(
                    format!("{{
                        \"success\": false,
                        \"payload\": \"{{}}\"
                    }}")
                )
        }

        let validation = Validation { sub: Some("authToken".to_owned()), ..Validation::default() };

        let auth_token = match decode::<AuthPayload>(
            req.cookie("authToken").unwrap().value(),
            &DecodingKey::from_secret(env::var("jwt_secret").unwrap().as_ref()), 
            &validation
        ) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            }
        };

        HttpResponse::Ok()
            .body(
                format!("{{
                    \"success\": true,
                    \"payload\": {{
                        \"username\": \"{}\"
                    }}
                }}", auth_token.claims.name)
            )
    }

    #[get("/logout")]
    pub async fn logout(req: HttpRequest) -> HttpResponse {
        const LOGOUT_MESSAGE: &'static str = "{{
            \"success\": true,
            \"message\": \"Successfully Logout\"
        }}";    

        if req.cookie("authToken").is_none() {
            return HttpResponse::Ok()
                .content_type("application/json")
                .body(
                    format!("{}", LOGOUT_MESSAGE)
                )
        }

        HttpResponse::Ok()
            .content_type("application/json")
            .del_cookie(&req.cookie("authToken").unwrap())
            .body(
                format!("{}", LOGOUT_MESSAGE)
            )
    }
}

pub mod html {
    use actix_web::{ Result, get };
    use actix_files::{ NamedFile };

    #[get("/")]
    pub async fn index() -> Result<NamedFile> {
        Ok(NamedFile::open("static/index.html")?)
    }

    #[get("/login")]
    pub async fn login() -> Result<NamedFile> {
        Ok(NamedFile::open("static/login.html")?)
    }
}