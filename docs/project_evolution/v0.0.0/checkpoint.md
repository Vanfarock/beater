# V.0.0

## Achievements

In this first version of the software, the basic structure was set up to send and retrieve data/commands to the GPU using buffers, queues and command pools

The idea is:
* Save a group of bytes into a buffer
* Send this buffer to the GPU
* Retrieve buffer data and use it to render an RGBA8 Image

Each pixel of the image has 4 bytes, where each one represents a part of RGBA

The result is saved in an image as follows:

![Result of first version execution](image.png)

## Code structure

So far the code reside in a single main function
