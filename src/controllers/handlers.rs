use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use sqlx::mysql::{MySqlPool};


#[derive(Serialize, Debug, sqlx::FromRow)]
struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
struct UserResponse {
    user: User,
    message: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}




#[get("/users/{user_id}")]
async fn get_user(path: web::Path<i32>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let user_id: i32 = path.into_inner(); 



    let user: Result<User, sqlx::Error> = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id=?",
        user_id
    ).fetch_one(pool.get_ref()).await;

    if user.is_err() {
        return HttpResponse::BadRequest().json(Response {
            message: "No user found with given id.".to_string()
        });
    }

    HttpResponse::Ok().json(UserResponse {
        user: user.unwrap(), 
        message: "Got user.".to_string(),
    })
}
