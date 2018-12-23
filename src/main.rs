#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(uniform_paths)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_json;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;
use diesel::RunQueryDsl;

// Used to Setup DB Pool
use rocket::Request;
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::http::Status;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::new(manager).expect("Failed to create pool")
}


use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

use rocket::State;
use rocket::fairing::AdHoc;


struct AngularDir(String);

#[get("/")]
fn index(angular_dir: State<AngularDir>) -> Option<NamedFile> {
	NamedFile::open(Path::new(&angular_dir.0).join("index.html")).ok()
}

#[get("/user/<id>", rank = 1)]
fn get_user(db: DB, id: i32) -> String{
  use schema::users::dsl::*;
  let connection = db.conn();
  let result = users.first::<models::User>(connection).expect("error loading user");
  println!("{:?}", result.email );
  String::from("ok!")
}

#[get("/<file..>")]
fn static_files(file: PathBuf, angular_dir: State<AngularDir>) -> Option<NamedFile>{
  NamedFile::open(Path::new(&angular_dir.0).join(file)).ok()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
      .mount("/", routes![index, get_user])
    	// .mount("/", routes![index, static_files, get_user])
    	.attach(AdHoc::on_attach("Angular Config", |rocket| {
    		let angular_dir = rocket.config()
    			.get_str("angular_root")
    			.unwrap_or("dist/frontend/")
    			.to_string();
    		println!("{:?}",angular_dir );
    		Ok(rocket.manage(AngularDir(angular_dir)))
    	}))
    	.launch();
}

lazy_static! {
  pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
  pub fn conn(&self) -> &PgConnection {
    &*self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
  type Error = r2d2::Error;
  fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    match DB_POOL.get() {
      Ok(conn) => Success(DB(conn)),
      Err(e) => Failure((Status::InternalServerError, e)),
    }
  }
}
