use mongodb::{
    bson::doc,
    sync::Collection,
};
use crate::restaurant::Restaurant;

// Update teh first match - what is first?
pub fn update_one(my_coll: &Collection<Restaurant>) -> mongodb::error::Result<()> {

    let filter = doc! { "name": "Cafe Shangirila" };
    let update = doc! { "$set": doc! {"cuisine": "Mysterious"} };
    let res = my_coll.update_one(filter, update).run()?;
    println!("\nUpdated documents: {}", res.modified_count);

    Ok(())
}
pub fn update_many(my_coll: &Collection<Restaurant>) -> mongodb::error::Result<()> {

    let filter = doc! { "name": "Cafe De Ja Vu" };
    // Adding a new field will require a new struct to accommodate it
    let update = doc! { "$set": doc! {"price": "$$$"} };
    let res = my_coll.update_many(filter, update).run()?;
    println!("\nUpdated documents: {}", res.modified_count);

    Ok(())
}
