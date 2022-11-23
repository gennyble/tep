# tep

<details>
	<summary>Why is it called tep?</summary>
<blockquote>
<p>me: it's called tep</p>
<p>friend: what inspired that? anything in particular?</p>
<p style="white-space: pre-wrap">me:
💭 TeX -> ReX
💭 Does TeX actually stand for anything? I want it to mean like Raster blah blah if TeX means Text blah blah.  
<i>checks wikipedia</i>
💭 No, it does not stand for anything.
💭 Well that's a deadend. But three-letter things are the kind-of-standard.
💭 What does it do? It lets you "typeset images" but that's stupid. It's for pixel art, mainly, because anything more would be tedious.
💭 Textual Pixel Editor
💭 tpe is unsayable. I need to vowel in the middle.
⭐ tep</p>
</blockquote>
</details>

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
#### `short gray`: `#1`
This expands to `#111111`

#### `long gray`: `#12`
This expands to `#121212`

#### `short rgb`: `#abc`
This expands to `#aabbcc`

#### `short rgba`: `#abcd`
This expands to `#aabbccdd`

#### `long rgb`: `#abcdef`
This is probably how you're used to seeing hex colours

#### `long rgba`: `#abcdef12`
This is probably how you're used to seeing RGBA in hex