#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    name: String,
    value: i32,
}

#[get("/api")] 
fn get_items() -> Json<Vec<Item>> {
    let items = vec![
        Item {
            name: "Item 1".to_string(),
            value: 10,
        },
        Item {
            name: "Item 2".to_string(),
            value: 20,
        },
    ];

    Json(items)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_items])
}

