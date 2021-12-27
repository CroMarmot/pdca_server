# https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen
#
# apt-get install protobuf-compiler
#
# cargo install protobuf-codegen
#
rm -r ./rust
mkdir rust
protoc --rust_out ./rust req.proto