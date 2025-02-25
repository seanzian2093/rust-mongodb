mod find;
mod client;
mod insert;
mod restaurant;
mod update;
mod replace;
mod delete;
mod distinct_values;

use mongodb::bson::{doc, Document};
use mongodb::sync::Collection;
use crate::restaurant::{Restaurant, Restaurant2};

static INSERT: bool = false;
// static INSERT: bool = true;

fn main() -> mongodb::error::Result<()> {
    let client = client::get_client()?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let my_coll2: Collection<Restaurant2> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let my_coll3: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    // Send a ping to confirm a successful connection
    client.database("admin").run_command(doc! { "ping": 1 }).run()?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    // Update
    // update::update_one(&my_coll)?;
    // update::update_many(&my_coll)?;

    // Insert
    if INSERT {

        let insert_structs = vec! [
            Restaurant {
                name: "While in Kathmandu".to_string(),
                cuisine: "Nepalese".to_string(),
            },
            Restaurant {
                // name: "Cafe Himalaya".to_string(),
                name: "Cafe Shangrila".to_string(),
                cuisine: "Nepalese".to_string(),
            }
        ];
        let insert_struct = Restaurant {
            name: "Cafe De Ja Vu".to_string(),
            cuisine: "French".to_string(),
        };

        insert::insert_many(&my_coll, insert_structs)?;
        insert::insert_one(&my_coll, insert_struct)?;

    }
    // Find records
    // find::find_one(&my_coll3, doc!{"cuisine": "Canadian"})?;
    find::find_many(&my_coll3, doc!{"name": "Cafe Shangrila"})?;

    // Replace record
    replace::replace_one(&my_coll3)?;
    find::find_many(&my_coll3, doc!{"name": "Cafe Shangirila"})?;
    find::find_many(&my_coll3, doc!{"name": "Cafe Shangrila"})?;

    // Delete
    delete::delete_one(&my_coll3, doc!{"name": "Cafe Shangirila"})?;
    delete::delete_many(&my_coll3, doc!{"name": "Cafe Shangirila"})?;
    find::find_many(&my_coll3, doc!{"name": "Cafe Shangirila"})?;

    // Count
    let ct = my_coll3.estimated_document_count().run()?;
    println!("Number of documents: {}", ct);
    let ct = my_coll3.count_documents(doc! { "name": doc! { "$regex": "Shang" } }).run()?;
    println!("Number of matching documents: {}", ct);

    // List distinct values
    distinct_values::get_distinct_values(&my_coll3, doc! {"name": "Cafe Shangrila"}, "cuisine")?;

    Ok(())
}
