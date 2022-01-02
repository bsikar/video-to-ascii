#! /bin/sh
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 ascii_inverted_color.mp4   -h 100 --show_ascii true --show_inverted true --show_color true
mpv ascii_inverted_color.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 ascii_inverted.mp4         -h 100 --show_ascii true --show_inverted true --show_color false
mpv ascii_inverted.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 ascii_color.mp4            -h 100 --show_ascii true --show_inverted false --show_color true
mpv ascii_color.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 ascii.mp4                  -h 100 --show_ascii true --show_inverted false --show_color false
mpv ascii.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 inverted_color.mp4         -h 100 --show_ascii false --show_inverted true --show_color true
mpv inverted_color.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 inverted.mp4               -h 100 --show_ascii false --show_inverted true --show_color false
mpv inverted.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 color.mp4                  -h 100 --show_ascii false --show_inverted false --show_color true
mpv color.mp4
cargo run --release -- assets/videos/big_buck_bunny_720p_1mb.mp4 none.mp4                   -h 100 --show_ascii false --show_inverted false --show_color false
mpv none.mp4
