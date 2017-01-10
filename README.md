Lantern
=======

Prototype of a disk based inverted index written in rust. Currently no way near being useful.

Goals
-----

To make a disk based inverted index, available as a `.so` (or `.dll` if you are on windows). By solving this part of the problem in a search cluster, the cluster mainly has to worry about the clustering. By making this library linkable, the cluster can be written in any language that can link out via FFI, thus removing the coupling to any specific runtime.

Things to do, before this is of any use
---------------------------------------

In no particular order:

- [ ] Make indices load up existing segments
- [ ] Restructure the index itself
- [ ] Make indices intelligently use several segments
- [ ] Make it possible to rewrite segments
- [ ] Support more operations on documents (update, delete)
- [ ] Support more complex types than just a string
- [ ] Support different ways to analyse text
- [ ] Query via a DSL, possibly structs
- [ ] Document it
- [ ] Pick a license
- [ ] Push to crates.io
- [ ] Also compile to a linkable

Contributing
------------

Are welcome! I am not very good at rust (yet?). So suggestions, corrections, and feature additions are very welcome.
