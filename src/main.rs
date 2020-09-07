#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::NamedFile;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies};
use diesel::prelude::*;
mod routes;
mod user;
/* Our extern crates */
#[macro_use] extern crate diesel;


extern crate dotenv;

/* Importing functions */
use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

/* Static files imports */
use std::path::{Path, PathBuf};


/* Will hold our data structs */
pub mod models;

use models::*;
/* auto-generated table macros */
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![routes::index,routes::index2,routes::files 
    ,user::singout,user::singout2,user::register,user::singin,user::singin2,user::login]).launch();
}