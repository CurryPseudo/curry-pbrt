#!/bin/bash
while true; do
	cargo run --release --example render_from_file $1
	inotifywait -qe close_write $1
done
