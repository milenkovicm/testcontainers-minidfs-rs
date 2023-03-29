use testcontainers::{core::WaitFor, Image, ImageArgs, RunnableImage};

pub const PORT_NAME_NODE: u16 = 9000;
pub const PORT_NAME_NODE_HTTP: u16 = 8020;

const PORT_DATA_N0DE_0: u16 = 50010;
const PORT_DATA_NODE_1: u16 = 50011;
const PORT_DATA_NODE_2: u16 = 50012;
const PORT_DATA_NODE_3: u16 = 50013;

pub struct MiniDFS {
    tag: String,
}

impl MiniDFS {
    /// Runnable docker image
    pub fn runnable() -> RunnableImage<MiniDFS> {
        Self::runnable_from_tag("latest".into())
    }

    /// Runnable docker image from a tag
    pub fn runnable_from_tag(tag: String) -> RunnableImage<MiniDFS> {
        RunnableImage::from(MiniDFS { tag })
            .with_mapped_port((PORT_NAME_NODE, PORT_NAME_NODE))
            .with_mapped_port((PORT_NAME_NODE_HTTP, PORT_NAME_NODE_HTTP))
            .with_mapped_port((PORT_DATA_N0DE_0, PORT_DATA_N0DE_0))
            .with_mapped_port((PORT_DATA_NODE_1, PORT_DATA_NODE_1))
            .with_mapped_port((PORT_DATA_NODE_2, PORT_DATA_NODE_2))
            .with_mapped_port((PORT_DATA_NODE_3, PORT_DATA_NODE_3))
        //.with_container_name("minidfs") // we wont name container as failed or exited may occupy the name
    }
}

#[derive(Debug, Default, Clone)]
pub struct MiniDFSArgs;

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
        self.tag.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::StdOutMessage {
            message: String::from("testcontainers.hdfs.status.READY"),
        }]
    }
}
