# video-to-ascii
A simple rust video to ascii converter

```sh
video_to_ascii 0.1.0
Brighton Sikarskie <bsikar@tuta.io>
A simple rust video to ascii converter

USAGE:
    video_to_ascii [OPTIONS] --input <INPUT>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <FILTER>                  The filter used to resize the image [default: nearest]  [possible values:
                                           nearest, triangle, catmullrom, gaussian, lanczos3]
    -h, --height <HEIGHT>                  The height of the output image
    -i, --input <INPUT>                    This is the video you are converting to ascii
    -o, --output <OUTPUT>                  This is the output file, if not specified, stdout is used
    -p, --pixel_size <PIXEL_SIZE>          The pixel size used to resize the image [default: 16]
    -a, --show_ascii <SHOW_ASCII>          Should the output show ascii or not [default: true]  [possible values: true,
                                           false]
    -c, --show_color <SHOW_COLOR>          Should the output show color or not [default: true]  [possible values: true,
                                           false]
    -x, --show_inverted <SHOW_INVERTED>    Should the output's color be inverted or not [default: false]  [possible
                                           values: true, false]
    -w, --width <WIDTH>                    The width of the output image
```

Be careful with you dimentions because you can make come VERY large frames, e.g this [20480x11520 frame](https://media.githubusercontent.com/media/bsikar/video-to-ascii/main/outputs/frame1.png)
 that I made by mistake. If your compiliation times are off the charts your dimentions might be too high, this one frame took me over an hour on my computer.

This program works by taking an input video such as [assets/videos/big_buck_bunny_720p_1mb.mp4](https://github.com/bsikar/video-to-ascii/blob/main/assets/videos/big_buck_bunny_720p_1mb.mp4) then applying flags on it


## input video




https://user-images.githubusercontent.com/65072072/162579826-14de0fb5-8b1b-4f5d-8c9d-89570b43bcb4.mov




## output videos

### terminal
```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -a true -x true -c true
```

### file

```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted_color.mp4 -h 100 -a true -x true -c true
```




```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted.mp4 -h 100 -a true -x true -c false
```


```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_color.mp4 -h 100 -a true -x false -c true
```



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii.mp4 -h 100 -a true -x false -c false
```



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted_color.mp4 -h 100 -a false -x true --c true
```



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted.mp4 -h 100 -a false -x true --c false
```


```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o color.mp4 -h 100 -a false -x false --c true
```



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o none.mp4 -h 100 -a false -x false -c false
```
