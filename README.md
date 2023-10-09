# Blob Graph

Represents a tree of blobs in a store.

Has two object types:

- Blob
- Node

A node contains arbitrary descriptions of other blobs and nodes that relate to it.

Both Nodes and Blobs may have meta data.

Needs:

- UID generator to identify new blobs that come in, unusally a hash
- Object store that allows adding and getting items by their UID
- Allow reading information for Node targets

The default implementation for reading nodes assumes they are a blob containing JSON data.

Node data and blob data need not be stored in the same object store, but they can be. 
