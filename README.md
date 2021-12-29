# LINC -- Linc Is Not Code

LINC is a prototype for a programming editor based on direct manipulation of
abstract syntax tree.

In LINC, everything is a tree, and editing is focused on interactively editing such trees.

A tree is represented as a Directed Acyclic Graph (DAG) starting from a root node.

Each node is represented by:

- a `kind`, which determines how its children pointers are to be interpreted
- a `value`, which is a byte string representing the node value, for leaf nodes
- a `links` (multi-)map, which maps each link name to zero or more children nodes; link names within the map are unordered, but the order of individual links with the same name is preserved

Nodes are content-addressed, meaning each node is immutable and uniquely identified by its own hash. Modifying a node is accomplished by creating a new node with the desired changes, and bubbling up its hash to the parent node, and so on, until reaching the root node, at which point a new root hash is produced, which summarizes the entire updated tree.

To allow meaningful structural editing of a tree, LINC relies on a schema that determines for each kind the name and molteplicity of its links with a given name.

Roughly speaking, a node kind corresponds to a struct, and link names correspond to its fields.

A tree may be used to represent a variety of structures, detailed below.

## Program ASTs

An AST of a programming language (e.g. Rust) may be represented as a LINC tree.

## JSON / YAML / protobuf objects

Trivial.

## Command-line arguments

When invoking a program from a command line shell, a number of parameters are passed to it, usually in the form of flags. The program then has to parse all those flags back into an abstract intenral representation, which is often severly limited by the fact that flags are textual objects and must be escaped correctly. But if we have the schema of the expected structure that a program is expecting, we should be able to directly create and manipulate this structure and pass it to the program directly, which would be safer and more expressive than traditional command line flags.
