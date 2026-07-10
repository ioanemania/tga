# TGA File Format

A .tga file is commonly used graphic file format that is easy to read and write.
Format Notes

- TGA files support 8,15,16,24,32 bit depths.
- 8 bit can be grey scale
- Runlength compression
- Additional Footer information optional x/y origin relative to lower left with origin in lower left
- descriptor: 00vhaaaa
    - h horizontal flip
    - v vertical flip
    - a alpha bits

# File Header (18 bytes)

```c
typedef struct
{
    byte  identsize;          // size of ID field that follows 18 byte header (0 usually)
    byte  colourmaptype;      // type of colour map 0=none, 1=has palette
    byte  imagetype;          // type of image 0=none,1=indexed,2=rgb,3=grey,+8=rle packed

    short colourmapstart;     // first colour map entry in palette
    short colourmaplength;    // number of colours in palette
    byte  colourmapbits;      // number of bits per palette entry 15,16,24,32

    short xstart;             // image x origin
    short ystart;             // image y origin
    short width;              // image width in pixels
    short height;             // image height in pixels
    byte  bits;               // image bits per pixel 8,16,24,32
    byte  descriptor;         // image descriptor bits (vh flip bits)

    // pixel data follows header

} TGA_HEADER
```

# Example File Header
Here is the header of a 32 bit image with dimensions 1920x1080. Note that these
hex rows are 18 bytes so the entire TGA header is on the top row.  Also notice
that even though the columapbits is set to 32 the bits is set to 24, thus the
pixel data is actually 24 bpp.

```
00000000 00 00 02 00 00 00 00 20 00 00 00 00 80 07 38 04 18 00 ....... ......8...
00000018 31 52 41 18 52 10 10 6A 18 31 6A 5A 39 73 5A 41 73 6A 1RA.R..j.1jZ9sZAsj
00000036 52 73 7B 52 73 9C 39 5A 7B 31 4A 6A 31 4A 73 41 6A 94 Rs{Rs.9Z{1Jj1JsAj.
00000054 52 73 A4 29 20 18 20 18 10 10 10 10 18 18 18 29 29 29 Rs.) . ........)))
00000072 20 29 29 29 31 31 29 31 31 29 39 39 39 39 39 31 31 39  )))11)11)99999119
00000090 20 31 31 39 39 39 31 41 41 31 39 41 31 39 39 31 39 39  119991AA19A199199
```
