
/* Import macros and others */
use crate::*;
/* For beeing able to serialize */
use rocket::request::{self, Request, FromRequest};
use crate::rocket::outcome::IntoOutcome;
use schema::*;
use serde::Serialize;
#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32, 
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser{
    pub first_name: String,
    pub last_name:String,
    pub password:String,
    pub email: String,
}
impl<'a, 'r> FromRequest<'a, 'r> for User{
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        print!("gola mundo");
        request.cookies()
        .get_private("user_id")
        .and_then(|cookie|
        { cookie.value().parse::<i32>().ok()})
        .map(|id| {
           let mut vector= users::table
        .select(users::all_columns)
        .filter(users::id.eq(id))
        .load::<User>(&crate::establish_connection())
        .expect("Something happned while retrieving the hero of this id");
        vector.remove(0)
        })
        .or_forward(())
    
    }
}
   