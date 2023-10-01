#!/bin/bash
# Cargo clean is not run, as this will remove timings history.

echo "Building debug and release..."
cargo build
cargo build --release --timings
bin="whatishedoing_learning_rust"

echo "Compressing..."
upx --best --lzma ./target/release/$bin

debug_size=$(du --bytes "./target/debug/$bin" | cut -f1)
release_size=$(du --bytes "./target/release/$bin" | cut -f1)
percent=$(( 100-(debug_size/release_size )))

printf "Finished: the release bin is %00.00f%% of the debug size!\n" "$percent"
