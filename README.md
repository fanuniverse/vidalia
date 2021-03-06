# Vidalia

An image processing service intended to be run in an isolated container.

It serves a single POST endpoint, `/`, which accepts
multipart requests with the following parameters:
* `manifest` (text), specifying which transformations to perform
* `image` (file), the source image

See *tests/* for supported transformations and usage examples.

## Docker

The image is available at [littlebobbytables/vidalia](https://hub.docker.com/r/littlebobbytables/vidalia/).

It is recommended that you pull a specific version tag;
*latest* follows the *master* branch, which may not always be stable.

## Credits

The idea of processing images in-memory has been suggested by
[liamwhite](https://github.com/liamwhite).
