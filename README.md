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

The spaces are optional but let it stay at least a little more square than it could be in whatever font this is than I'm using.

Install it with
```
cargo install tepimg
```
or clone it down and
```
cargo install --path tep
```

And you use it like this:
```
tepimg tests/standard.tep tests/standard.png
```

Or you can save the palette in a different file and do:
```
tepimg -p tests/standard_palette tests/standard.tep tests/standard.tep
```

Do note that if the input tep file has a palette in it, the one passed with `-p` will be used instead.

### Valid colour definitions
short gray: `#1` becomes `#111111`  
long gray: `#12` becomes `#121212`  
short rgb: `#abc` becomes `#aabbcc`  
short rgba: `#abcd` becomes `#aabbccdd`  
long rgb: `#abcdef`  
long rgba: `#abcdef12`  
`rgb(...)` and `rgba(...)` with bytes. So like, `rgb(51,170,136)`