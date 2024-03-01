image.ppm: clean_image
	cargo run >> image.ppm
clean_image:
	rm -f image.ppm
test:
	cargo test