use mongodb::{
    bson::Document,
    sync::Collection,
};

// Find first matching  - what is first?
pub fn find_one(my_coll: &Collection<Document>, filter: Document) -> mongodb::error::Result<()> {
    let result = my_coll.find_one(filter).run()?;

    println!("\nFind one result: \n{:?}", result);

    Ok(())
}

// Use `Document` to accommodate heterogeneous structs
pub fn find_many(my_coll: &Collection<Document>, filter: Document) -> mongodb::error::Result<()> {

    let cursor = my_coll.find(filter).run()?;
    println!("\nFind multiple results:");
    for result in cursor {
        println!("{:?}", result?);
    }

    Ok(())
}
