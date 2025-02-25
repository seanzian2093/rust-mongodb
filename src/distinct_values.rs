use mongodb::{
    bson::Document,
    sync::Collection,
};

pub fn get_distinct_values(my_coll: &Collection<Document>, filter: Document, field: &str) -> mongodb::error::Result<()> {

    let distinct_values= my_coll.distinct(field, filter).run()?;
    println!("\nList of field values for '{}':", field);
    for b in distinct_values.iter() {
        println!("{:?}", b);
    }
    Ok(())

}
