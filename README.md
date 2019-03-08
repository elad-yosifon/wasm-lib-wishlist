# Motivation
Creating a WebAssembly framework/library that will provide common functions and data-structures that can be highly optimized if implemented natively.

# Wishlist:

## hash functions:
- [x] crc32
- [x] crc64
- [ ] md5
- [ ] sha-1
- [ ] sha-256

# utils
- [ ] ~~base64 with a custom lookup table~~ (waiting for custom configs feature on [rust-base64](https://github.com/alicemaz/rust-base64/issues/87))
- [ ] ~~nanotime~~ (system time is not supported)

## compression
- [ ] gzip and it's variants
- [ ] brotli

## data-structures and algorithms:
- [ ] trie tree
- [ ] binary search

## serializers:
- [ ] parquet
- [ ] protobuf
- [ ] flatbuffers
