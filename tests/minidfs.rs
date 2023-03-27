use hdfs_native::HdfsRegistry;
use log::info;
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

const DATA: &str = "1234567890";

#[tokio::test]
async fn e2e() {
    let _ = env_logger::builder().is_test(true).try_init();
    std::env::set_var("LIBHDFS3_CONF", "libhdfs3-hdfs-client.xml");
    let docker = clients::Cli::default();

    let container = MiniDFS::runnable();

    let server_node = docker.run(container);

    let fs_registry = HdfsRegistry::new();
    let hdfs_server_url = format!(
        "hdfs://{}:{}/",
        "localhost",
        server_node.get_host_port_ipv4(PORT_NAMENODE)
    );

    info!("HDFS Name Note to be used: [{}]", hdfs_server_url);
    let fs = fs_registry.get(&hdfs_server_url).expect("creation of registry");

    let test_dir = format!("/{}", "D0");
    info!("Directory used for this tests: [{}]", test_dir);

    fs.mkdir(&test_dir).expect("root dir created");
    assert!(fs.exist(&test_dir));

    let test_file = format!("{}/{}", test_dir, "F0");
    info!("File used for basic file operations: [{}]", test_file);

    let f = fs.create(&test_file).expect("file open for writing");
    assert!(f.is_writable());
    assert!(!f.is_readable());

    f.write(DATA.as_bytes()).expect("data to be written");
    f.flush();
    f.close().expect("file to be closed");

    let f = fs.append(&test_file).expect("file open for append");
    f.write(DATA.as_bytes()).expect("data to be appended");

    f.flush();
    f.close().expect("file to be closed");

    let f = fs.open(&test_file).expect("file open");
    assert!(!f.is_writable());
    assert!(f.is_readable());

    let mut buf = vec![0; 4 * DATA.len()];

    let len = f.read(&mut buf).expect("data to be read");

    let result = &buf[..len];
    assert_eq!(format!("{}{}", DATA, DATA).as_bytes(), result);

    fs.delete(&test_dir, true).expect("directory to be deleted");
}
