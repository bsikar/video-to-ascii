#! /bin/sh
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted_color.mp4   -h 100 --show_ascii true --show_inverted true --show_color true
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted.mp4         -h 100 --show_ascii true --show_inverted true --show_color false
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_color.mp4            -h 100 --show_ascii true --show_inverted false --show_color true
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii.mp4                  -h 100 --show_ascii true --show_inverted false --show_color false
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted_color.mp4         -h 100 --show_ascii false --show_inverted true --show_color true
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted.mp4               -h 100 --show_ascii false --show_inverted true --show_color false
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o color.mp4                  -h 100 --show_ascii false --show_inverted false --show_color true
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o none.mp4                   -h 100 --show_ascii false --show_inverted false --show_color false
