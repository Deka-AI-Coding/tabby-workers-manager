use bollard::Docker;

pub async fn list_images() -> Option<Vec<String>> {
    let docker = Docker::connect_with_socket_defaults().ok()?;

    let images = docker.list_images::<String>(None).await.ok()?;

    for image in &images {
        println!("{:?}", image.repo_tags);
    }

    Some(images.into_iter().map(|x| x.id).collect())
}
