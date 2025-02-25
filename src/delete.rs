use mongodb::{
    bson::Document,
    sync::Collection,
};

// Delete first matching  - what is first?
pub fn delete_one(my_coll: &Collection<Document>, filter: Document) -> mongodb::error::Result<()> {
    let result = my_coll.delete_one(filter).run()?;
    println!("\nDelete multiple results: {}", result.deleted_count);

    Ok(())
}

pub fn delete_many(my_coll: &Collection<Document>, filter: Document) -> mongodb::error::Result<()> {
    let result = my_coll.delete_many(filter).run()?;
    println!("\nDelete multiple results: {}", result.deleted_count);

    Ok(())
}
