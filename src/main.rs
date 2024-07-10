#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{
    serde::json::{json, Json, Value},
    State,
};
use rocket::{Request, Response};

use std::env;
use std::ops::DerefMut;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::futures::lock::Mutex;
use uuid::Uuid;

use crate::models::Member;

mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct DatabaseConnection {
    pub connection: Mutex<PgConnection>,
}

impl DatabaseConnection {
    pub fn new() -> Self {
        let connection = establish_connection();
        Self {
            connection: Mutex::new(connection),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/member")]
pub async fn get_member(connection: &State<DatabaseConnection>) -> Value {
    let results = schema::member::table
        .select(models::Member::as_select())
        .load::<Member>(connection.connection.lock().await.deref_mut())
        .expect("Error loading members");
    json!(results)
}

#[put("/member", data = "<member>")]
pub async fn put_member(member: Json<Member>, connection: &State<DatabaseConnection>) {
    diesel::insert_into(schema::member::table)
        .values(member.0)
        .execute(connection.connection.lock().await.deref_mut())
        .expect("Error saving new member");
}

#[get("/attendant")]
pub async fn get_attendant(connection: &State<DatabaseConnection>) -> Value {
    let results = schema::attendant::table
        .select(models::Attendant::as_select())
        .load::<models::Attendant>(connection.connection.lock().await.deref_mut())
        .expect("Error loading attendants");
    json!(results)
}

#[put("/attendant", data = "<attendant>")]
pub async fn put_attendant(
    attendant: Json<models::Attendant>,
    connection: &State<DatabaseConnection>,
) {
    let mut attendant = attendant.into_inner();
    attendant.registered_at = chrono::Local::now();
    diesel::insert_into(schema::attendant::table)
        .values(attendant)
        .execute(connection.connection.lock().await.deref_mut())
        .expect("Error saving new attendant");
}

#[delete("/attendant/<id>")]
pub async fn delete_attendant(id: String, connection: &State<DatabaseConnection>) {
    let id = Uuid::parse_str(&id).unwrap();
    diesel::delete(schema::attendant::table.find(id))
        .execute(connection.connection.lock().await.deref_mut())
        .expect("Error deleting attendant");
}
#[options("/attendant")]
pub fn attendant_preflight() -> String {
    String::new()
}
#[options("/attendant/<_id>")]
pub fn attendant_id_preflight(_id: String) -> String {
    String::new()
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PUT, DELETE, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn launch() -> _ {
    let database_connection = DatabaseConnection::new();

    rocket::build()
        .manage(database_connection)
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                get_member,
                put_member,
                get_attendant,
                put_attendant,
                delete_attendant,
                attendant_preflight,
                attendant_id_preflight,
            ],
        )
}
