# MiniDFS cluster

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

Limitations:

- ports are hardcoded, thus only single instance per host is possible
