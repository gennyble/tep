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

Valid color definitions:  
##### `short gray`: `#1`
This expands to `#111111`

##### `long gray`: `#12`
This expands to `#121212`

##### `short rgb`: `#abc`
This expands to `#aabbcc`

##### `short rgba`: `#abcd`
This expands to `#aabbccdd`

##### `long rgb`: `#abcdef`
This is probably how you're used to seeing hex colours

##### `long rgba`: `#abcdef12`
This is probably how you're used to seeing RGBA in hex

##### `rgb(...)` and `rgba(...)` (with bytes, so like, `rgb(127, 53, 14)`)