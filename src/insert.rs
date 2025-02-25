use mongodb::sync::Collection;
use crate::restaurant::Restaurant;
pub fn insert_many(my_coll: &Collection<Restaurant>, insert_structs: Vec<Restaurant>) -> mongodb::error::Result<()> {
    let insert_many_result = my_coll.insert_many(insert_structs).run()?;
    println!("\nInserted documents with _ids:");
    for (_key, value) in &insert_many_result.inserted_ids {
        println!("{}", value);
    }

    Ok(())
}
pub fn insert_one(my_coll: &Collection<Restaurant>, insert_struct: Restaurant) -> mongodb::error::Result<()> {
    let insert_one_result = my_coll.insert_one(insert_struct).run()?;
    println!("\nInserted documents with _id:\n {}", insert_one_result.inserted_id);

    Ok(())
}
