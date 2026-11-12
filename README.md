# Native OMML models

This crate contains all 186 models from the checksummed OMML runtime
declaration graph and their generated XML codecs. The public `models`
and `xml_codecs` modules expose the statically typed model API.
The optional `model-serde` feature adds generated model JSON and the C ABI
in `include/omml.h`.

The source requires the isolated LutaML boxed XML candidate recorded in
the producer receipt. A local Cargo patch selects that candidate.
The shared XML parser applies its default bounded parsing policy.
OMML scalar bindings are copied from the source package. DateTime uses
Chrono for the qualified ISO lexical profile. Ruby's broader
DateTime.parse forms receive a typed codec refusal.

The C ABI borrows inputs, returns one owned JSON reply and provides an
explicit release function. Each successful reply contains model JSON
and the generated XML serialization. Empty and nil fields follow the
generated serializer's presence policy.
