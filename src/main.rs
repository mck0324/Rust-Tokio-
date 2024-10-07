use tokio::time::{sleep, Duration};
use log::Level;

// fn fib(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 => 1,
//         n => fib(n - 2) + fib(n - 1),
//     }
// }

// async fn sleeper() {
//     log::info!("Sleeping");
//     time::sleep(time::Duration::from_secs(1)).await;
//     log::info!("Awake!");
// }

// async fn reader() {
//     log::info!("Reading some been data");
//     let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
//     let mut contents = vec![];
//     f.read_to_end(&mut contents).await.unwrap();
//     log::info!("Read beeg {}  bytes", contents.len());

//     fib(40);
// }

// async fn run() {
//     tokio::join!(
//         sleeper(),
//         reader(),
//     );
// }
// fn main() {
//     simple_logger::init_with_level(Level::Info).unwrap();

//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let future = run();
//     rt.block_on(future);
// }

async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    sleep(Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}

async fn run() {
    tokio::spawn(async {
        sleeper("first").await;
    });
    sleeper("second").await;
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();
    println!("Took {:?} seconds", end - start);
}