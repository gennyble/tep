I made this because I didn't have an image editor of any sort on my machine and I just needed to make a few little pixel art things for a project I was working on. Instead of trying to find an image editor for MacOS that I liked, I just made this. It's small and cute and I like it and I hope you do, too.

tep lets you make little pixel art images in a file and then compile them to images. See:
```
w: #FFF
b: #000

w b w b w b
b w b w b w
w b w b w b
b w b w b w
w b w b w b
b w b w b w
```

The spaces are optional but let it stay at least a little more square than it could be in whatever font this is that I'm using.

Install it with
```
cargo install tepimg
```
or clone it down and
```
cargo install --path tep
```

### Usage

```
tepimg tests/standard.tep tests/standard.png
```

Or, if you want to share a palette between images, you can pass it in separate. The palette provided with `-p` will be used instead of any palette in the image.

```
tepimg -p tests/standard_palette tests/standard.tep tests/standard.png
```

### Valid colour definitions
short gray: `#1` becomes `#111111`  
long gray: `#12` becomes `#121212`  
short rgb: `#abc` becomes `#aabbcc`  
short rgba: `#abcd` becomes `#aabbccdd`  
long rgb: `#abcdef`  
long rgba: `#abcdef12`  
`rgb(...)` and `rgba(...)` with bytes. So like, `rgb(51,170,136)`