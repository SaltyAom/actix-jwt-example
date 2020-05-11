pub mod api {
    use actix_web::{ HttpResponse, get, post, web, cookie, HttpRequest };
    use serde::{ Serialize, Deserialize };

    use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
    use jsonwebtoken::errors::ErrorKind;

    use crate::libs::{ get_time, get_cookie };

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
            &EncodingKey::from_secret("Koisuru Fortune Cookie".as_ref())
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
        let auth_token = get_cookie(req, "authToken");

        let validation = Validation { sub: Some("authToken".to_owned()), ..Validation::default() };

        let auth_token = match decode::<AuthPayload>(
            &auth_token,
            &DecodingKey::from_secret("Koisuru Fortune Cookie".as_ref()), 
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
    pub async fn logout() -> HttpResponse {
        let auth_cookie = cookie::Cookie::build("authToken", "")
            .path("/")
            .http_only(true)
            .finish();

        HttpResponse::Ok()
            .content_type("application/json")
            .del_cookie(&auth_cookie)
            .body(
                format!("{{
                    \"success\": true,
                    \"message\": \"Successfully Logout\"
                }}")
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