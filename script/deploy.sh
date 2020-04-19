#! /bin/bash
rsync_create_directory () {
	dir=$(dirname $1)
	mkdir -p $2$dir
	rsync -arP $1 $2$1
}
( git status --short| grep '^?' | cut -d\  -f2- && git ls-files ) | sort -u | while read file
do 
	rsync_create_directory $file ~/bebby/curry-pbrt/
done
output_file=$(ssh 192.168.0.101 "cd h/curry-pbrt; cargo run --release --example render_from_file $1")
cp ~/bebby/curry-pbrt/$output_file image/
