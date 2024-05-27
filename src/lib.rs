#![doc = include_str!("../README.md")]

use log::{debug, info};
use std::path::PathBuf;
use testcontainers::{
    core::{Mount, WaitFor},
    Image, ImageArgs, RunnableImage,
};

/// Namenode Port
pub const PORT_NAME_NODE: u16 = 9000;
/// Namenode HTTP Port
pub const PORT_NAME_NODE_HTTP: u16 = 8020;

const PORT_DATA_N0DE_0: u16 = 50010;
const PORT_DATA_NODE_1: u16 = 50011;
const PORT_DATA_NODE_2: u16 = 50012;
const PORT_DATA_NODE_3: u16 = 50013;
const PORT_KERBEROS: u16 = 62600;

pub struct MiniDFS {
    tag: String,
    config_path: Option<PathBuf>,
    kerberos_enabled: bool,
}

pub struct MiniDFSBuilder {
    tag: String,
    kerberos_enabled: bool,
    config_volume: bool,
}

impl MiniDFSBuilder {
    /// Select tag for testcontainer
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag = tag.to_string();
        self
    }

    /// Enable exposing hadoop configuration as as local volume
    pub fn with_config_volume(mut self, enabled: bool) -> Self {
        self.config_volume = enabled;

        self
    }

    /// enable kerberos support
    /// kerberos support will expose kerberos configuration and files
    /// as a local volume
    pub fn with_kerberos(mut self, enabled: bool) -> Self {
        self.kerberos_enabled = enabled;

        self
    }
    /// local directory where kerberos configuration
    /// directory should be mounted.
    fn project_root_directory() -> PathBuf {
        // TODO check override env as well
        // CARGO_TARGET_DIR

        // this one does not work as expected
        // let mut cwd = std::env::current_dir().expect("cwd");
        let mut root = project_root::get_project_root().expect("project root folder");

        root.push("target");
        root.push("HDFS");

        root
    }

    pub fn build(self) -> RunnableImage<MiniDFS> {
        let config_path = if self.kerberos_enabled || self.config_volume {
            Some(Self::project_root_directory())
        } else {
            None
        };

        let mut image = RunnableImage::from(MiniDFS {
            tag: self.tag,
            config_path: config_path.clone(),
            kerberos_enabled: self.kerberos_enabled,
        })
        .with_mapped_port((PORT_NAME_NODE, PORT_NAME_NODE))
        .with_mapped_port((PORT_NAME_NODE_HTTP, PORT_NAME_NODE_HTTP))
        .with_mapped_port((PORT_DATA_N0DE_0, PORT_DATA_N0DE_0))
        .with_mapped_port((PORT_DATA_NODE_1, PORT_DATA_NODE_1))
        .with_mapped_port((PORT_DATA_NODE_2, PORT_DATA_NODE_2))
        .with_mapped_port((PORT_DATA_NODE_3, PORT_DATA_NODE_3))
        .with_mapped_port((PORT_KERBEROS, PORT_KERBEROS));

        if self.config_volume || self.kerberos_enabled {
            let volume_kerberos_path = config_path.unwrap();
            let volume_kerberos = volume_kerberos_path.to_string_lossy();
            if self.kerberos_enabled {
                // we need to create dir if does not exist
                // ignoring error, as it may already exist

                info!(
                    "Kerberos support has been enabled, testcontainer will mount container volume to local directory: [{}]",
                volume_kerberos
            );
            } else {
                info!(
                    "Configuration support has been enabled, testcontainer will mount container volume to local directory: [{}]",
                volume_kerberos
            );
            }
            let target_dir_creation = std::fs::create_dir(&volume_kerberos_path);
            debug!("local directory creation error: {} ", target_dir_creation.is_err());

            let volume_hdfs = Mount::bind_mount(volume_kerberos, "/tmp/HDFS");
            image = image.with_mount(volume_hdfs)
        }

        if self.kerberos_enabled {
            image = image
                .with_env_var(("KDC_ENABLED", "true"))
                .with_env_var(("HOST_OS", std::env::consts::OS))
        }

        image
    }
}

impl MiniDFS {
    pub fn builder() -> MiniDFSBuilder {
        MiniDFSBuilder {
            tag: "latest".into(),
            kerberos_enabled: false,
            config_volume: false,
        }
    }

    /// Runnable docker image
    pub fn runnable() -> RunnableImage<MiniDFS> {
        Self::builder().build()
    }

    /// Runnable docker image from a tag
    pub fn runnable_from_tag(tag: &str) -> RunnableImage<MiniDFS> {
        Self::builder().with_tag(tag).build()
    }
    // core-site.xml
    pub fn kerberos_config(&self) -> Option<PathBuf> {
        if !self.kerberos_enabled {
            None
        } else if std::env::consts::OS == "macos" {
            // macos handles TCP protocol configuration a slightly
            // different, thus in case of macos we use prepared
            // configuration
            self.config_path.clone().map(|mut p| {
                p.push("macos_krb5.conf");
                p
            })
        } else {
            self.config_path.clone().map(|mut p| {
                p.push("krb5.conf");
                p
            })
        }
    }

    pub fn kerberos_cache(&self) -> Option<PathBuf> {
        if !self.kerberos_enabled {
            None
        } else {
            self.config_path.clone().map(|mut p| {
                p.push("krb5_cache");
                p
            })
        }
    }

    pub fn hdfs_config(&self) -> Option<PathBuf> {
        self.config_path.clone().map(|mut p| {
            p.push("core-site.xml");
            p
        })
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
            message: "testcontainers.hdfs.status.READY".into(),
        }]
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Enable RUST_LOG logging configuration for test
    let _ = env_logger::builder().is_test(true).try_init();
}
