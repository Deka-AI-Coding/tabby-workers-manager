extern crate tabby_worker_manager;
extern crate tokio;

mod service;
use env_logger::Env;
use service::{prelude::*, Service};

mod config;

mod user;
use user::AuthenticatedUser;

use actix_web::{get, middleware::Logger, put, web, App, HttpServer, Responder};
use clap::Parser;
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

#[put("/{name}/start")]
async fn start_worker(
    service: web::Data<Service>,
    path_params: web::Path<String>,
    _: AuthenticatedUser,
) -> Result<impl Responder, tabby_worker_manager::Error> {
    let name = path_params.into_inner();
    let worker = service.start_worker(&name).await?;

    #[derive(Serialize)]
    struct Worker {
        worker: WorkerInfo,
    }

    Ok(web::Json(Worker { worker }))
}

#[put("/{name}/stop")]
async fn stop_worker(
    service: web::Data<Service>,
    path_params: web::Path<String>,
    _: AuthenticatedUser,
) -> Result<impl Responder, tabby_worker_manager::Error> {
    let name = path_params.into_inner();
    let worker = service.stop_worker(&name).await?;

    #[derive(Serialize)]
    struct Worker {
        worker: WorkerInfo,
    }

    Ok(web::Json(Worker { worker }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::parse();
    let port = config.port;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let zookeeper = TabbyWorkerZookeeper::default();
        let service = Service::new(zookeeper);

        App::new()
            .app_data(web::Data::new(service))
            .app_data(web::Data::new(config.clone()))
            .service(get_workers)
            .service(get_worker)
            .service(start_worker)
            .service(stop_worker)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
