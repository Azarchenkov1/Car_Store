#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate image;
//extern crate formdata;

use serde::de::DeserializeOwned;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Error;
use std::fs::File;
use std::fs::FileType;
use std::io::prelude::*;
//use formdata::{FormData, FilePart};
use rocket::data::FromDataSimple;
use rocket::data::FromData;
use rocket::request::Form;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
pub use image::ImageFormat::JPEG;
use rocket::data::Data;

#[derive(FromForm)]
struct Article {
    name: String
}

//configuration
pub static is_initialize: bool = false;
pub static is_server_start: bool = true;

//structure
#[derive(Serialize, Deserialize, Default)]
struct Vehicle {
    id: i8,
    modelname: String,
    engine: String,
    mileage: i32,
    price: i32,
    photo: String
}

#[derive(Serialize, Deserialize, Default)]
struct Model (
    Vehicle,Vehicle,Vehicle,Vehicle,Vehicle,
    Vehicle,Vehicle,Vehicle,Vehicle
);

//initialize model.json
pub fn initialize() -> Result<(), Error>
{
    println!("initialize...");

    let vehicle1 = Vehicle {
        id: 1.to_owned(),
        modelname: "Subaru Forester".to_owned(),
        engine: "2.0".to_owned(),
        mileage: 10000.to_owned(),
        price: 8000.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle2 = Vehicle {
        id: 2.to_owned(),
        modelname: "Porshe Panamera".to_owned(),
        engine: "3.0 diesel".to_owned(),
        mileage: 104000.to_owned(),
        price: 43800.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle3 = Vehicle {
        id: 3.to_owned(),
        modelname: "Audi A6 Allroad".to_owned(),
        engine: "2.5".to_owned(),
        mileage: 260000.to_owned(),
        price: 8500.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle4 = Vehicle {
        id: 4.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle5 = Vehicle {
        id: 5.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle6 = Vehicle {
        id: 6.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle7 = Vehicle {
        id: 7.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle8 = Vehicle {
        id: 8.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let vehicle9 = Vehicle {
        id: 9.to_owned(),
        modelname: "null".to_owned(),
        engine: "null".to_owned(),
        mileage: 0.to_owned(),
        price: 0.to_owned(),
        photo: "null".to_owned()
    };

    let model = Model (
        vehicle1,
        vehicle2,
        vehicle3,
        vehicle4,
        vehicle5,
        vehicle6,
        vehicle7,
        vehicle8,
        vehicle9
    );

    println!("Initialize model...");
    
    println!("model.vehicle_id_1.modelname {}",model.0.modelname);
    println!("model.vehicle_id_1.modelname {}",model.1.modelname);
    println!("model.vehicle_id_2.modelname {}",model.2.modelname);
    println!("model.vehicle_id_3.modelname {}",model.3.modelname);
    println!("model.vehicle_id_4.modelname {}",model.4.modelname);
    println!("model.vehicle_id_5.modelname {}",model.5.modelname);
    println!("model.vehicle_id_6.modelname {}",model.6.modelname);
    println!("model.vehicle_id_7.modelname {}",model.7.modelname);
    println!("model.vehicle_id_8.modelname {}",model.8.modelname);

    //json serialize
    let json = serde_json::to_string(&model)?;

    //output
    println!("json serialize... {}", json);

    let mut file = File::create("views/model.json")
    .expect("Error: file can not be created");

    file.write_all(json.as_bytes())
    .expect("Error: data can not be pushed");

    println!("file: model.json is ready");
    Ok(())
}

pub fn model()
{
    println!("model ->");
    if(is_initialize)
    {
        initialize();
    }
}

#[post("/", format = "multipart", data = "<formdata>")]
fn photo(formdata: Form<Article>) -> JsonValue
{
    json!({"status": "ok"})
}

#[post("/", format = "json", data = "<model>")]
fn new(model: String) -> JsonValue
{
    println!("received {}", model);

    let mut file = File::create("views/model.json")
    .expect("Error: file can not be created");

    file.write_all(model.as_bytes())
    .expect("Error: data can not be pushed");

    json!({"status": "ok"})
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", StaticFiles::from("views"))
    .mount("/json", routes![new])
    .mount("/photo", routes![photo])
}

fn main() {
    println!("main ->");
    model();
    if(is_server_start)
    {
        rocket().launch();
    }
}