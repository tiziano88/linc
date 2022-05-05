use crate::{schema, types::Node};
use std::marker::PhantomData;

trait NodeStore {
    fn get(&self, digest: &str) -> Option<&Node>;
}

struct Link<'s, S: NodeStore, T: FromRawNode> {
    _marker: PhantomData<T>,
    digest: Digest,
    node_store: &'s S,
}

impl<'s, S: NodeStore, T: FromRawNode> Link<'s, S, T> {
    fn get(&self) -> Option<T> {
        self.node_store
            .get(&self.digest)
            .and_then(|node| T::from_raw_node(node))
    }
}

trait FromRawNode: Sized {
    fn from_raw_node(raw_node: &Node) -> Option<Self>;
}

struct Root<'s, S: NodeStore> {
    node_store: &'s S,
    raw_node: Node,
}

impl<'s, S: NodeStore> Root<'s, S> {
    fn item_vec(&self) -> Vec<Link<'s, S, RootItem>> {
        self.raw_node.links[&0]
            .iter()
            .map(|digest| Link {
                _marker: PhantomData,
                digest: digest.clone(),
                node_store: self.node_store.clone(),
            })
            .collect()
    }
    // fn item(&self, index: usize) -> Option<RootItem> {}
    // fn item_remove(&self, index: usize) -> Option<RootItem> {}
    // fn item_insert(&self, index: usize, RootItem) -> Option<RootItem> {}
}

enum RootItem {
    Git(Git),
    Docker(Docker),
    RustFragment(RustFragment),
    GoFragment(GoFragment),
    MarkdownFragment(MarkdownFragment),
}

impl From<Git> for RootItem {
    fn from(git: Git) -> Self {
        RootItem::Git(git)
    }
}

impl From<Docker> for RootItem {
    fn from(docker: Docker) -> Self {
        RootItem::Docker(docker)
    }
}

impl FromRawNode for RootItem {
    fn from_raw_node(raw_node: &Node) -> Option<Self> {
        match raw_node.kind.as_str() {
            schema::GIT => Git::from_raw_node(raw_node).map(Into::into),
            schema::DOCKER => Docker::from_raw_node(raw_node).map(Into::into),
            schema::RUST_FRAGMENT => RustFragment::from_raw_node(raw_node).map(Into::into),
            _ => None,
        }
    }
}

struct Docker {}

impl Docker {
    fn command(&self) -> Vec<DockerCommand> {}
}

impl FromRawNode for Docker {
    fn from_raw_node(raw_node: &Node) -> Option<Self> {
        Some(Docker {})
    }
}

enum DockerCommand {
    Build(DockerBuild),
    Run(DockerRun),
}

struct DockerBuild {}

impl DockerBuild {
    fn add_host(&self) {}
    fn build_arg(&self) {}
    fn cache_from(&self) {}
    fn compress(&self) {}
    fn file(&self) {}
    fn label(&self) {}
}

struct DockerRun {}

impl DockerRun {
    fn attach(&self) {}
    fn cap_add(&self) {}
    fn cap_drop(&self) {}
    fn detach(&self) {}
}

struct Git<'s, S: NodeStore> {
    node_store: &'s S,
    raw_node: Node,
}

impl<'s, S: NodeStore> Git<'s, S> {}

impl FromRawNode for Git {
    fn from_raw_node(raw_node: &Node) -> Option<Self> {
        (raw_node.kind.as_str() == schema::GIT).then_some(Git {})
    }
}

struct RustFragment {}

impl FromRawNode for RustFragment {
    fn from_raw_node(raw_node: &Node) -> Option<Self> {
        Some(RustFragment {})
    }
}

struct GoFragment {}

struct MarkdownFragment {}
