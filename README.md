# Image Hider
An application in Rust that encodes files into PNG images

This project uses the Rust [image crate](https://github.com/PistonDevelopers/image) to encode arbitrary data into image files.

## Why?
This provides an easy way to share files. Now you can use image hosts or imageboards.

Also, shits and giggles and a proof-of-concept.

## Usage
To encode a file:

    $ image_hider --enc filename
    $ eog filename.png

To decode a file:

    $ image_hider --dec filename.png
    $ xdg-open filename

To specify a non-default file name:

    $ image_hider --enc filename -o output.png
    $ eog output.png

## Example Output
Here's Alice in Wonderland encoded as an image.

![Alice in Wonderland](https://i.imgur.com/Ppk4O3O.png)

Note that the images must be stored in a lossless format. Make sure to check your image host's policy on compression.

## How it works
Rust's image crate is capable of taking an arbitrary vector of bytes and interpretting it as pixel data. In this case, RGBA data was chosen so as to minimize the image dimensions, since each pixel contains four bytes of data.

To avoid the issue of deciding on appropriate image dimensions, I decided to make all images square. Thus the dimensions are determined through the equation

![\left \lceil \sqrt{\frac{size}{4} + 1} \right \rceil](http://latex.codecogs.com/svg.latex?%5Cinline%20%5Ctiny%20%5Cleft%20%5Clceil%20%5Csqrt%7B%5Cfrac%7Bsize%7D%7B4%7D%20&plus;%201%7D%20%5Cright%20%5Crceil)

This leads to a problem, however; in order to ensure that the file is square, padding must be added to the end. In order to know exactly where the padding begins, the first pixel is set aside to contain the original file size. Essentially, the file's size is stored as a 32 bit unsigned integer, and through bitwise operations, it is decomposed into four bytes. These bytes are prepended to the vector containing the provided file's data, and then the necessary padding is done.

This vector is then fed into a PNGEncoder, which interprets the bytes as RGBA data.

To decode a file, the process is essentially reversed.