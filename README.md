# LINC -- Linc Is Not Code

Interactive demo: https://tiziano88.github.io/linc

LINC is a prototype for a general-purpose editor based on direct manipulation of abstract tree data structures; it aims to be the Vim / Emacs for editing any form of tree-like structured data.

LINC should be embeddable in other systems (e.g. other editors, websites, databases, etc.), instead of each system reinventing structural editing in different ways, and it should be extendable via a schema language that determines how to interpret and manipulate the structure of the trees.

## Trees

A tree is represented as a Directed Acyclic Graph (DAG) starting from a root node, with links pointing to child nodes.

Nodes are content-addressed, meaning each node is immutable and uniquely identified by its own hash. The entire structure is a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree). This also structurally ensures that there cannot be any cycles while traversing a tree.

Each node is represented by:

- a `kind`, which determines how its links are to be interpreted; this is a UUID that refers to a concrete schema definition for that kind, which is used to interpret the links of that node
- a `value`, which is a byte string representing the node value, for leaf nodes
- a `links` (multi-)map, which maps each link id to zero or more children nodes; link names within the map are sorted by id in order to have a canonical representation, but the order of individual links with the same link id is preserved

##  Node example

Here is an example node:

```
"2db14f2d5133a6403aa6d763b733548f1fbe33a778c0df023731af952645621b": {
    kind: "33ac449e-bbae-44f1-bcae-fa85f1b93e67",
    value: "",
    links: {
        1: [
            "3061849bd6c36a380c4f9d66aab7e9c93836372df03d00a8fe63b96a4b5ba2e4",
        ],
        4: [
            "1145f2e2ddf7c539a446bc54a2e49ea8ec53a1a955a6b263b63a42b02f070eb3",
        ],
    },
},
```

First of all, this node is uniquely and permanently identified by its SHA256 hash `2db14f2d5133a6403aa6d763b733548f1fbe33a778c0df023731af952645621b`. Given this hash, only this particular node representation would correspond to it.

Roughly speaking, a node corresponds to a struct, and links correspond to pointers to its fields.

The node has kind `33ac449e-bbae-44f1-bcae-fa85f1b93e67`; this id is used to look up the concrete schema for that kind, in this case: https://github.com/tiziano88/linc/blob/b8c71c8e35885bf0a56dfb8a31c77725b1c39f27/src/schema.rs#L184-L221:

```rust
    "33ac449e-bbae-44f1-bcae-fa85f1b93e67" => DOCKER_BUILD @ Kind {
        name: "docker_build",
        fields: hashmap!{
            0 => Field {
                name: "add-host",
                raw: true,
                ..Default::default()
            },
            1 => Field {
                name: "build-arg",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            2 => Field {
                name: "cache-from",
                raw: true,
                repeated: true,
                ..Default::default()
            },
            3 => Field {
                name: "compress",
                raw: true,
                ..Default::default()
            },
            4 => Field {
                name: "file",
                raw: true,
                ..Default::default()
            },
            5 => Field {
                name: "label",
                raw: true,
                ..Default::default()
            },
        },
        ..Default::default()
    },
```

From here we can see that this kind represents a `docker_build` struct with various fields.

The advantage of using a UUID instead of a human readable string to represent the kind is that the schema may be renamed without affecting all the nodes that refer to that kind. For instance, this kind may be renamed to `docker_command_build` (or any other name) in the future, without affecting any of the existing nodes that refer to the kind by its id.

From the kind definition, we can see that the struct has a number of fields. Fields are identified by numbers, for the same reason as above.

While kind ids need to be globally unique (hence the UUID format), field numbers only need to be unique within a kind definition.

Field names are used for pretty-printing and for documentation, but are not stored in the serialized tree structure.

Note that it is necessary to statically know the correct schema in order to be able to semantically intepret a node and its links, though it is possible to structurally traverse a node even without knowing the schema.

Modifying a node is accomplished by creating a new node with the desired changes, and bubbling up its hash to its parent node, and so on recursively, until reaching the root node, at which point a new root hash is produced, which summarizes the entire updated tree.

A tree may be used to represent a variety of structures, detailed below.

## Program ASTs

An AST of a programming language (e.g. Rust) may be represented as a LINC tree.

## JSON / YAML / protobuf objects

Trivial.

## Command-line arguments

When invoking a program from a command line shell, a number of parameters are passed to it, usually in the form of flags. The program then has to parse all those flags back into an abstract intenral representation, which is often severly limited by the fact that flags are textual objects and must be escaped correctly. But if we have the schema of the expected structure that a program is expecting, we should be able to directly create and manipulate this structure and pass it to the program directly, which would be safer and more expressive than traditional command line flags.
