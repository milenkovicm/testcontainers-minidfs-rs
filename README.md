# MiniDFS Cluster

[![github action](https://github.com/milenkovicm/testcontainers-minidfs-rs/actions/workflows/basic.yml/badge.svg)](https://github.com/milenkovicm/testcontainers-minidfs-rs/actions/workflows/basic.yml)
[![Crates.io](https://img.shields.io/crates/v/testcontainers-minidfs-rs)](https://crates.io/crates/testcontainers-minidfs-rs)
[![Crates.io](https://img.shields.io/crates/d/testcontainers-minidfs-rs)](https://crates.io/crates/testcontainers-minidfs-rs)

Testcontainer HDFS cluster using hadoop mini DFS.

```rust
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

let docker = clients::Cli::default();
let container = MiniDFS::runnable();
let server_node = docker.run(container);

let hdfs_namenode_url = format!("hdfs://{}:{}/", "localhost", server_node.get_host_port_ipv4(PORT_NAME_NODE));
```

HDFS name node should be available at `hdfs://localhost:9000` and name node http at `http://localhost:8020`.

## Kerberos Support

MiniHDFS can be configured with kerberos support:

```rust, no_run
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

let docker = clients::Cli::default();
let container = MiniDFS::builder().with_kerberos(true).build();
let server_node = docker.run(container);
```

MiniDFS will be configured to support kerberos and all required files will be exposed as a docker volume mounted in the target directory.

```rust, no_run
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

let container = MiniDFS::builder().with_kerberos(true).build();
// pre-populated kerberos cache file (ccache)
let kerberos_cache = container.inner().kerberos_cache();
// kerberos configuration file krb5.conf
let kerberos_config = container.inner().kerberos_config();
// hadoop configuration core-site.xml
let hdfs_config = container.inner().hdfs_config();
```

All required files needed for hdfs client setup are exposed. (`kinit` will be executed by the container, kerberos cache will be produced).

## Limitations

- ports are hardcoded, thus only single instance per host is possible
