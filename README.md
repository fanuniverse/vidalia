# Vidalia

An image processing service intended to be run in an isolated container.

It serves a single POST endpoint, `/`, which accepts
multipart requests with the following parameters:
* `manifest` (text), specifying which transformations to perform
* `image` (file), the source image

See *tests/* for supported transformations and usage examples.

## Credits

The idea of processing images in-memory has been suggested by
[liamwhite](https://github.com/liamwhite).
