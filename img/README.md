When we generate the oseda template from the static files, most of the files and stuff that we are copying will be plaintext. The notable exception here is the header. All the other stuff should already be embedded into the `npm init` binary. To actually embed this into the code, we have a few options. I really wish there was a library like Golangs `embed`, but the best I've found since then is using `xxd`. The `-i` flag will output the hex dump into C source code.

So doing
```bash
xxd -i default-header.jpg > ../src/default_header.c
```
gives you the image as hex data in C Source like
```c
unsigned char header_jpg[] = {
  0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, 0x4a, 0x46, 0x49, 0x46, 0x00, 0x01,
  0x01, 0x01, 0x01, 0x2c, 0x01, 0x2c, 0x00, 0x00, 0xff, 0xe1, 0x00, 0x56,
  ...
  0x50, 0x08, 0xd1, 0x1c, 0x50, 0x06, 0xc5, 0x00, 0x65, 0x00, 0x17, 0x6a,
  0x00, 0x1a, 0x00, 0xca, 0x00, 0xde, 0x68, 0x03, 0xff, 0xd9
};
unsigned int header_jpg_len = 81538;
```
Which is basically exactly what we need. We can then write that data to a header.jpg on the user's system, since we can't embed the image into the binary.

This process can is automated through the `embed-img.sh` script.
```bash
./embed-img default_header.jpg -o ../src
```
