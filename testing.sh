#! /bin/sh
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_inverted_color.mov   -h 100 --show_ascii true --show_inverted true --show_color true
mpv ascii_inverted_color.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_inverted.mov         -h 100 --show_ascii true --show_inverted true --show_color false
mpv ascii_inverted.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_color.mov            -h 100 --show_ascii true --show_inverted false --show_color true
mpv ascii_color.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii.mov                  -h 100 --show_ascii true --show_inverted false --show_color false
mpv ascii.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o inverted_color.mov         -h 100 --show_ascii false --show_inverted true --show_color true
mpv inverted_color.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o inverted.mov               -h 100 --show_ascii false --show_inverted true --show_color false
mpv inverted.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o color.mov                  -h 100 --show_ascii false --show_inverted false --show_color true
mpv color.mov
cargo run --release -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o none.mov                   -h 100 --show_ascii false --show_inverted false --show_color false
mpv none.mov
