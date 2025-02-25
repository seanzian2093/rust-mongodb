use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Restaurant {
    pub name: String,
    pub cuisine: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Restaurant2 {
    pub name: String,
    pub cuisine: String,
    pub price: String,
}
