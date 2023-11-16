use log::info;
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

#[tokio::test]
#[serial_test::serial]
async fn e2e_nameode_hello() {
    let _ = env_logger::builder().is_test(true).try_init();

    let docker = clients::Cli::default();
    let container = MiniDFS::runnable();
    let server_node = docker.run(container);
    let hdfs_server_url = format!(
        "hdfs://{}:{}/",
        "localhost",
        server_node.get_host_port_ipv4(PORT_NAME_NODE)
    );

    let client = hdfs_native::Client::new(&hdfs_server_url).expect("HDFS client to be created");

    // not great test but should verify that name node can be connected
    let path = "/test";
    client.mkdirs(path, 777, true).await.expect("directory to be created");
    let file_info = client.get_file_info(path).await.expect("file info to be retrieved");

    assert!(file_info.isdir)
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_kerberos() {
    let docker = clients::Cli::default();

    let container = MiniDFS::builder().with_kerberos(true).build();
    let kerberos_cache = container.inner().kerberos_cache();
    let kerberos_config = container.inner().kerberos_config();
    let hdfs_config = container.inner().hdfs_config();

    let _server_node = docker.run(container);

    assert!(kerberos_cache.is_some());
    assert!(kerberos_config.is_some());
    assert!(hdfs_config.is_some());

    let kerberos_cache = kerberos_cache.unwrap();
    let kerberos_config = kerberos_config.unwrap();
    let hdfs_config = hdfs_config.unwrap();

    info!("kerberos cache: {:?}", kerberos_cache);
    info!("kerberos config: {:?}", kerberos_config);
    info!("hdfs config: {:?}", hdfs_config);

    assert!(kerberos_cache.exists());
    assert!(kerberos_config.exists());
    assert!(hdfs_config.exists());
}

#[tokio::test]
#[serial_test::serial]
async fn e2e_config_volume() {
    let docker = clients::Cli::default();
    let container = MiniDFS::builder().with_config_volume(true).build();

    let hdfs_config = container.inner().hdfs_config();
    let _server_node = docker.run(container);

    assert!(hdfs_config.is_some());
    let hdfs_config = hdfs_config.unwrap();
    info!("hdfs config: {:?}", hdfs_config);
    assert!(hdfs_config.exists());
}
#[cfg(test)]
#[ctor::ctor]
fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}
