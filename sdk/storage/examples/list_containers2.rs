use azure_core::HttpClient;
use azure_storage::clients::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();

    let response = storage_client
        .as_container_client("azuresdkforrust")
        .list_blobs()
        .execute()
        .await?;
    println!("key response = {:#?}", response);

    // we can still use the storage_client since it's not moved
    // by as_container_client above
    // (rather reference counted)
    let response = storage_client.list_containers().execute().await?;
    println!("key response = {:#?}", response);

    // let's test a SAS token
    // the code is identical
    // once instantiated
    let sas_token = "?sv=2019-12-12&ss=bfqt&srt=sco&sp=rwdlacupx&se=2020-12-05T20:20:58Z&st=2020-12-05T12:20:58Z&spr=https&sig=vxUuKjQW4%2FmB884f%2BdqCp4h3O%2BYuYgIJN8RVGHFVFpY%3D";
    let client = StorageAccountClient::new_sas_token(http_client.clone(), &account, sas_token)
        .as_storage_client();
    let response = client.list_containers().execute().await?;
    println!("sas response = {:#?}", response);

    Ok(())
}
