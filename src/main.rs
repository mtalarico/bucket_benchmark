mod comparison;

use comparison::DB;
use std::error::Error;

const NUM_MEASUREMENTS: u32 = 100000;

// Atlas Credentials
const USERNAME: &str = "michael";
const PASSWORD: &str = "mongodb";
const CLUSTERNAME: &str = "playground";
const PROJECT_HASH: &str = "vz1ep";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = format!(
        "mongodb+srv://{USERNAME}:{PASSWORD}@{CLUSTERNAME}.{PROJECT_HASH}.mongodb.net/?retryWrites=true&w=majority"
    );
    let dal = DB::connect(uri.as_str()).await?;
    dal.insert_standalone().await?;
    dal.insert_bucketed().await?;

    return Ok(());
}
