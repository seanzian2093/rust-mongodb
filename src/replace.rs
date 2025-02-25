use mongodb::{
    bson::doc,
    sync::Collection,
};
use mongodb::bson::Document;

// Replace the first match - what is first?
pub fn replace_one(my_coll: &Collection<Document>) -> mongodb::error::Result<()> {

    let filter = doc! { "name": "Cafe Shangrila" };
    let replace= doc! {
        "name": "Cafe Shangrila",
        "cuisine": "Mysterious/Fusion",
        "price": "$$$$",
    };
    let res = my_coll.replace_one(filter, replace).run()?;
    println!("\nReplaced documents: {}", res.modified_count);

    Ok(())
}
