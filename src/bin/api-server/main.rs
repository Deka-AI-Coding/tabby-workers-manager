extern crate tabby_worker_manager;
extern crate tokio;

mod service;
use service::{prelude::*, Service};

use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
use tabby_worker_manager::TabbyWorkerZookeeper;

#[get("/")]
async fn get_workers(
    service: web::Data<Service>,
) -> Result<impl Responder, tabby_worker_manager::Error> {
    let workers = service.get_workers().await?;

    #[derive(Serialize)]
    struct Workers {
        workers: Vec<WorkerInfo>,
    }

    Ok(web::Json(Workers { workers }))
}

#[get("/{name}")]
async fn get_worker(
    service: web::Data<Service>,
    path_params: web::Path<String>,
) -> Result<impl Responder, tabby_worker_manager::Error> {
    let name = path_params.into_inner();
    let worker = service.get_worker(&name).await?;

    #[derive(Serialize)]
    struct Worker {
        worker: WorkerInfo,
    }

    Ok(web::Json(Worker { worker }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let zookeeper = TabbyWorkerZookeeper::default();
        let service = Service::new(zookeeper);

        App::new()
            .app_data(web::Data::new(service))
            .service(get_workers)
            .service(get_worker)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
