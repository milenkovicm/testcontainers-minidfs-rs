# MiniDFS Cluster

Testcontainer local HDFS cluster using hadoop mini dfs.

```rust
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

let docker = clients::Cli::default();
let container = MiniDFS::runnable();
let server_node = docker.run(container);

let hdfs_server_url = format!("hdfs://{}:{}/", "localhost", server_node.get_host_port_ipv4(PORT_NAME_NODE));
```

HDFS name node should be available at `hdfs://localhost:9000` and name node http at `http://localhost:8020`.

## Kerberos Support

MiniHDFS can be configured with kerberos enabled:

```rust
use testcontainers::clients;
use testcontainers_minidfs_rs::*;

let docker = clients::Cli::default();
let container = MiniDFS::builder().with_kerberos(true).build();
let server_node = docker.run(container);
```

MiniDFS will be configured to support kerberos and all required files will be exposed as a docker volume mounted in the target directory.

```rust
let container = MiniDFS::builder().with_kerberos(true).build();
let kerberos_cache = container.inner().kerberos_cache();
let kerberos_config = container.inner().kerberos_config();
let hdfs_config = container.inner().hdfs_config();
```

All required files needed for hdfs client setup are exposed. (`kinit` will be executed by the container, kerberos cache will be produced).

## Limitations

- ports are hardcoded, thus only single instance per host is possible
