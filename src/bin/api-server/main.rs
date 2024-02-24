extern crate tabby_worker_manager;
extern crate tokio;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use tabby_worker_manager::{TabbyWorkerState, TabbyWorkerZookeeper};

#[derive(Serialize)]
struct WorkerInfo {
    name: String,
    state: TabbyWorkerState,
}

#[get("/")]
async fn get_workers(
    zookeeper: web::Data<TabbyWorkerZookeeper>,
) -> Result<impl Responder, tabby_worker_manager::Error> {
    let zookeeper = zookeeper.into_inner();
    let names = zookeeper.manager_names().await?;

    let mut workers = vec![];
    for name in names {
        let manager = zookeeper.manager(&name).await?;
        let state = manager.state().await?;
        workers.push(WorkerInfo { name, state });
    }

    #[derive(Serialize)]
    struct Workers {
        workers: Vec<WorkerInfo>,
    }

    Ok(web::Json(Workers { workers }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let zookeeper = TabbyWorkerZookeeper::default();

        App::new()
            .app_data(web::Data::new(zookeeper))
            .service(get_workers)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
