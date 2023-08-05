use std::sync::Arc;

use iot_server::database::{
    create_user, get_user_by_username, requests::UserRequest, responses::UserResponse,
};
use rocket_dyn_templates::Template;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    tokio::task::spawn_blocking,
    State,
};
use sqlx::{Pool, Sqlite, SqlitePool};

#[macro_use]
extern crate rocket;

const COOKIE_NAME: &str = "quid";

#[post("/register", data = "<request>")]
async fn post_register(
    pool: &State<Pool<Sqlite>>,
    request: Form<UserRequest>,
) -> Result<Json<UserResponse>, Status> {
    let UserRequest { username, password } = request.into_inner();

    let user_salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let password = argon2::hash_encoded(
        password.as_bytes(),
        user_salt.as_bytes(),
        &argon2::Config::default(),
    )
    .unwrap();

    let user = create_user(pool, &username, &password).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::Conflict),
    }
}

#[post("/login", data = "<request>")]
async fn post_login(
    pool: &State<Pool<Sqlite>>,
    cookies: &CookieJar<'_>,
    request: Form<UserRequest>,
) -> Result<Json<UserResponse>, Status> {
    let UserRequest { username, password } = request.into_inner();
    let user = Arc::new(match get_user_by_username(pool, &username).await {
        Ok(user) => user,
        Err(_) => return Err(Status::Unauthorized),
    });
    let user_clone = user.clone();

    let valid = spawn_blocking(move || {
        argon2::verify_encoded(&user_clone.password, password.as_bytes()).unwrap()
    })
    .await
    .unwrap();

    match valid {
        true => {
            let user = Arc::try_unwrap(user).unwrap();

            cookies.add_private(Cookie::new(COOKIE_NAME, user.username.clone()));
            Ok(Json(user))
        }
        false => Err(Status::Unauthorized),
    }
}

#[get("/me")]
async fn get_me(
    pool: &State<Pool<Sqlite>>,
    cookies: &CookieJar<'_>,
) -> Result<Json<UserResponse>, Status> {
    let username = match cookies.get_private(COOKIE_NAME) {
        Some(username) => username,
        None => return Err(Status::Unauthorized),
    };

    let user = get_user_by_username(pool, username.value()).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/logout")]
async fn post_logout(cookies: &CookieJar<'_>) -> Result<Status, Status> {
    match cookies.get_private(COOKIE_NAME) {
        Some(_) => {
            cookies.remove_private(Cookie::named(COOKIE_NAME));
            Ok(Status::Ok)
        }
        None => Err(Status::Unauthorized),
    }
}

#[get("/")]
fn get_index() -> Template {
    Template::render("index", ())
}

#[get("/login")]
fn get_login() -> Template {
    Template::render("login", ())
}

#[get("/register")]
fn get_register() -> Template {
    Template::render("register", ())
}

#[launch]
async fn rocket() -> _ {
    let pool = SqlitePool::connect("sqlite://database.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    rocket::build()
        .mount("/", routes![get_index, get_login, get_register, post_register, post_login, get_me, post_logout])
        .manage(pool)
        .attach(Template::fairing())
}
