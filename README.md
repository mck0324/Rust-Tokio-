# Rust-Tokio-
tokio는 어플리케이션이 파일 읽기와 같은 작업을 기다리는 비차단 IO에 적합
또는 네트워크 데이터이지만 어플리케이션의 병목현상이 CPU 또는 성능 제한인 상황이 있을때 tokio는 기본 이벤트 루프에 단일 스레드를 사용하므로 작업이 과도한 CPU기반 작업을 수행하는 경우 실행중인 다른 비동기 작업의 속도가 실제로 느려짐


async fn sleeper(name: &str) {
    log::info!("{}: Sleeping", name);
    sleep(Duration::from_secs(1)).await;
    log::info!("{}: Awake!", name);
}

async fn run() {
    sleeper("first");
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
해당 코드를 실행시켜보면 second만 실행되는걸 볼 수 있다.
Future는 기본적으로 구문 설탕 역할을 하는 async 키워드를 사용할때마다 암시적으로 반환됨.