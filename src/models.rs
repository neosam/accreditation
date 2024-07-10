use chrono::Local;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::member)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Member {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub member_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::attendant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Attendant {
    pub id: Uuid,
    pub member_id: Uuid,
    pub registered_at: chrono::DateTime<Local>,
}
