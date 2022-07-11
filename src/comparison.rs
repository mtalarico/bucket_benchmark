use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, UpdateOptions},
    Client,
};
use rand::{distributions::Uniform, prelude::Distribution};
use sensor_data::SensorData;
use tokio::time::Instant;

use crate::NUM_MEASUREMENTS;

mod sensor_data;

pub struct DB {
    client: Client,
}

impl DB {
    async fn ping(client: &Client) -> Result<Document, mongodb::error::Error> {
        let result = client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        Ok(result)
    }

    pub async fn connect(uri: &str) -> Result<DB, mongodb::error::Error> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;

        DB::ping(&client).await?;

        return Ok(DB { client });
    }

    pub async fn insert_standalone(&self) -> Result<(), mongodb::error::Error> {
        let coll = self
            .client
            .database("test")
            .collection::<SensorData>("inserSensorData");
        let mut total = 0;

        let _ = coll.drop(None).await;

        for _ in 0..NUM_MEASUREMENTS {
            let data = get_rand_num();
            let start = Instant::now();
            let _ = coll.insert_one(SensorData::new(data, 0), None).await?;
            let duration = start.elapsed();
            total += duration.as_micros();
        }

        let avg = total / NUM_MEASUREMENTS as u128;
        print_results("Insert", &avg);

        return Ok(());
    }

    pub async fn insert_bucketed(&self) -> Result<(), mongodb::error::Error> {
        let coll = self
            .client
            .database("test")
            .collection::<SensorData>("updates");
        let mut total = 0;

        let _ = coll.drop(None).await;

        for _ in 0..NUM_MEASUREMENTS {
            let data = get_rand_num();
            let sensor_data = SensorData::new(data, 0);
            let start = Instant::now();
            let _ = coll
                .update_one(
                    doc! { "num_docs": { "$lt": 200}},
                    doc! { "$push": { "measurements": { "ts": sensor_data.ts, "data": sensor_data.data, "thread": sensor_data.thread}}, "$inc": { "num_docs": 1}},
                    UpdateOptions::builder().upsert(true).build(),
                )
                .await?;
            let duration = start.elapsed();
            total += duration.as_micros();
        }

        let avg = total / NUM_MEASUREMENTS as u128;
        print_results("Update", &avg);

        return Ok(());
    }
}

fn get_rand_num() -> i32 {
    let mut rng = rand::thread_rng();
    let die = Uniform::<i32>::from(1..100);
    return die.sample(&mut rng);
}

fn print_results(method: &str, avg: &u128) {
    println!("{method} method took an average of {avg} μs over {NUM_MEASUREMENTS} inserSensorData");
}
