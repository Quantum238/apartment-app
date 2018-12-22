#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};

#[macro_use] extern crate rocket;
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;
use rocket::State;
use rocket::fairing::AdHoc;

struct AngularDir(String);

#[get("/")]
fn index(angular_dir: State<AngularDir>) -> Option<NamedFile> {
	NamedFile::open(Path::new(&angular_dir.0).join("index.html")).ok()
}

#[get("/<file..>")]
fn static_files(file: PathBuf, angular_dir: State<AngularDir>) -> Option<NamedFile>{
	NamedFile::open(Path::new(&angular_dir.0).join(file)).ok()
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index, static_files])
    	.attach(AdHoc::on_attach("Angular Config", |rocket| {
    		let angular_dir = rocket.config()
    			.get_str("angular_root")
    			.unwrap_or("../apartment-app/src/")
    			.to_string();
    		println!("{:?}",angular_dir );
    		Ok(rocket.manage(AngularDir(angular_dir)))
    	}))
    	.attach(Template::fairing())
    	.launch();
}