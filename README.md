# video-to-ascii
A simple rust video to ascii converter

```
video-to-ascii 0.1.0
Brighton Sikarskie <bsikar@tuta.io>
A simple rust video to ascii converter

USAGE:
    video-to-ascii [OPTIONS] --input <INPUT>

OPTIONS:
    -a, --show_ascii <SHOW_ASCII>
            Should the output show ascii or not [default: true] [possible values: true, false]

    -c, --show_color <SHOW_COLOR>
            Should the output show color or not [default: true] [possible values: true, false]

    -f, --filter <FILTER>
            The filter used to resize the image [default: nearest] [possible values: nearest,
            triangle, catmullrom, gaussian, lanczos3]

    -h, --height <HEIGHT>
            The height of the output image
            If no height is given then it will scale the height based off the specified width, but
            if no width is specified then the input video's dimensions will be used

        --help
            Print help information

    -i, --input <INPUT>
            This is the video you are converting to ascii

    -o, --output <OUTPUT>
            This is the output file, if not specified, stdout is used

    -p, --pixel_size <PIXEL_SIZE>
            The pixel size used to resize the image [default: 16]

    -V, --version
            Print version information

    -w, --width <WIDTH>
            The width of the output image
            If no width is given then it will scale the width based off the specified height, but if
            no height is specified then the input video's dimensions will be used

    -x, --show_inverted <SHOW_INVERTED>
            Should the output's color be inverted or not [default: false] [possible values: true,
            false]
```

Be careful with you dimentions because you can make come VERY large frames, e.g this [20480x11520 frame](https://media.githubusercontent.com/media/bsikar/video-to-ascii/main/outputs/frame1.png)
 that I made by mistake. If your compiliation times are off the charts your dimentions might be too high, this one frame took me over an hour on my computer.

This program works by taking an input video such as [assets/videos/big_buck_bunny_720p_1mb.mp4](https://github.com/bsikar/video-to-ascii/blob/main/assets/videos/big_buck_bunny_720p_1mb.mp4) then applying flags on it


## input video
The input video can be any video format


https://user-images.githubusercontent.com/65072072/162579826-14de0fb5-8b1b-4f5d-8c9d-89570b43bcb4.mov


## output videos
The output video van be any format regardless of the input video's format. The video can be either played in the terminal or saved to a file.
If the video is played in the terminal its frame rate is going to be based off how fast the terminal can draw the characters.

### terminal
```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -a true -x true -c true
```

https://user-images.githubusercontent.com/65072072/162585904-2ae5d064-58cd-41cb-9162-198100f9668e.mov

### file

```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted_color.mp4 -h 100 -a true -x true -c true
```



https://user-images.githubusercontent.com/65072072/162687128-09ed07f6-2ef6-4ed9-b86d-010db4ee83ed.mov



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_inverted.mp4 -h 100 -a true -x true -c false
```


https://user-images.githubusercontent.com/65072072/162688266-2557589d-999d-496e-bf8c-b29adbc15643.mov



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii_color.mp4 -h 100 -a true -x false -c true
```


https://user-images.githubusercontent.com/65072072/162688374-b1a58383-827a-4c7b-b085-d7c369fb7498.mov




```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o ascii.mp4 -h 100 -a true -x false -c false
```


https://user-images.githubusercontent.com/65072072/162688385-d4d2936b-bccc-47fe-9638-a42fab7289fd.mov




```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted_color.mp4 -h 100 -a false -x true --c true
```



https://user-images.githubusercontent.com/65072072/162688410-37018932-08f3-49a8-be7d-59ee66fa2fb4.mov



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o inverted.mp4 -h 100 -a false -x true --c false
```


https://user-images.githubusercontent.com/65072072/162687595-17c19185-5eef-45db-b1ae-95a14a35789a.mov



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o color.mp4 -h 100 -a false -x false --c true
```


https://user-images.githubusercontent.com/65072072/162688432-57d8bd8d-4a2c-4f75-99cc-92be58c0e648.mov



```sh
cargo run -- -i assets/videos/big_buck_bunny_720p_1mb.mp4 -o none.mp4 -h 100 -a false -x false -c false
```


https://user-images.githubusercontent.com/65072072/162688510-db94c4c7-7e09-45e2-98cd-0b689b1d46c2.mov



