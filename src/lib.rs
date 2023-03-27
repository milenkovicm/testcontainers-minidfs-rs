use testcontainers::{core::WaitFor, Image, ImageArgs, RunnableImage};

pub const PORT_NAMENODE: u16 = 9000;
pub const PORT_NAMENODE_HTTP: u16 = 8020;

pub const PORT_DATANONDE_0: u16 = 50010;
pub const PORT_DATANONDE_1: u16 = 50011;
pub const PORT_DATANONDE_2: u16 = 50012;
pub const PORT_DATANONDE_3: u16 = 50013;

pub struct MiniDFS {}

impl MiniDFS {
    pub fn runnable() -> RunnableImage<MiniDFS> {
        RunnableImage::from(MiniDFS {})
            .with_mapped_port((PORT_NAMENODE, PORT_NAMENODE))
            .with_mapped_port((PORT_NAMENODE_HTTP, PORT_NAMENODE_HTTP))
            .with_mapped_port((PORT_DATANONDE_0, PORT_DATANONDE_0))
            .with_mapped_port((PORT_DATANONDE_1, PORT_DATANONDE_1))
            .with_mapped_port((PORT_DATANONDE_2, PORT_DATANONDE_2))
            .with_mapped_port((PORT_DATANONDE_3, PORT_DATANONDE_3))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MiniDFSArgs {}

impl ImageArgs for MiniDFSArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        Box::new(vec![].into_iter())
    }
}

impl Image for MiniDFS {
    type Args = MiniDFSArgs;

    fn name(&self) -> String {
        "milenkovicm/testcontainer-hdfs".into()
    }

    fn tag(&self) -> String {
        "latest".into()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::StdOutMessage {
            message: String::from("testcontainers.hdfs.status.READY"),
        }]
    }
}
