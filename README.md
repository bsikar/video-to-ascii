# video-to-ascii
A simple rust video to ascii converter

```
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
                                           If no height is given then it will scale the height based off the specified
                                           width, but if no width is specified then the input video's dimensions will be
                                           used
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
                                           If no width is given then it will scale the width based off the specified
                                           height, but if no height is specified then the input video's dimensions will
                                           be used
```

Be careful with you dimentions because you can make come VERY large frames, e.g this [20480x11520 frame](https://media.githubusercontent.com/media/bsikar/video-to-ascii/main/outputs/frame1.png)
 that I made by mistake. If your compiliation times are off the charts your dimentions might be too high, this one frame took me over an hour on my computer.

This program works by taking an input video such as [assets/videos/big_buck_bunny_720p_1mb.mov](https://github.com/bsikar/video-to-ascii/blob/main/assets/videos/big_buck_bunny_720p_1mb.mov) then applying flags on it


## input video
The input video can be any video format


https://user-images.githubusercontent.com/65072072/162579826-14de0fb5-8b1b-4f5d-8c9d-89570b43bcb4.mov


## output videos
The output video van be any format regardless of the input video's format. The video can be either played in the terminal or saved to a file.
If the video is played in the terminal its frame rate is going to be based off how fast the terminal can draw the characters.

### terminal
```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov
```

https://user-images.githubusercontent.com/65072072/162585904-2ae5d064-58cd-41cb-9162-198100f9668e.mov



### file

```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_inverted_color.mov -h 100 -a true -x true -c true
```






```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_inverted.mov -h 100 -a true -x true -c false
```




```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii_color.mov -h 100 -a true -x false -c true
```





```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o ascii.mov -h 100 -a true -x false -c false
```






```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o inverted_color.mov -h 100 -a false -x true --c true
```






```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o inverted.mov -h 100 -a false -x true --c false
```





```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o color.mov -h 100 -a false -x false --c true
```





```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mov -o none.mov -h 100 -a false -x false -c false
```


