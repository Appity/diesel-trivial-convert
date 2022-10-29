use dotenv::dotenv;

use diesel_trivial_convert::db;
use diesel_trivial_convert::models::ExampleModel;

fn main() {
    dotenv().ok();

    let mut conn = db::conn();

    let example = ExampleModel::create(&mut conn, "test".try_into().unwrap()).unwrap();

    println!("Created: {}", example.id());
}
