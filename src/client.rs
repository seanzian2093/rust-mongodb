use mongodb::sync::Client;

pub fn get_client() -> mongodb::error::Result<Client> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://127.0.0.1:27017/";

    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri)?;

    Ok(client)
}
