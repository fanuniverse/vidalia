# Vidalia

Assuming the following directory hierarchy (see `config/main.go`):
```
VIDALIA_PATH
├── cache/
│   ├── cached_image.png
│   ├── cached_animation.gif
│   └── ...
└── images/
    ├── png_image_id/
    |   ├── source.png      <-- the original image
    |   ├── thumbnail.png   <-- a 300px-wide version
    |   └── preview.png     <-- a 1280px-wide version
    └── gif_image_id/
        ├── source.gif      <-- the original animation
        ├── rendered.mp4    <-- an MP4/h264 version
        ├── rendered.webm   <-- a WebM/vp9 version
        └── poster.jpg      <-- a JPG of the first frame
```

Upon receiving a message with the following body (JSON-encoded)
from the **`image.process` queue on the default exchange**:
```
{"id": "10", "file": "randomcachestring.png"}
```
The service:
1) moves `VIDALIA_PATH/cache/randomcachestring.png`
to `VIDALIA_PATH/images/10/source.png`
2) analyzes image properties
3) creates preview and thumbnail versions in
`VIDALIA_PATH/images/10`

Analyzed image properties are then published to the
**`image.processed` queue on the default exchange** as a
JSON object with the following properties:
```
{
  "path":    (string)  path to the source.ext file in the images/ directory
  "ext":     (string)  image extension (png/jpg/gif)
  "id":      (string)  unique image identifier
  "width":   (uint)    image width
  "height":  (uint)    image height
  "phash":   (uint64)  phash
}
```

## Development

```bash
docker-compose run vidalia
```

Inside the container:

```bash
cd /go/src/vidalia

# Test
go test ./test

# Build and run
go build
./vidalia
```

### RabbitMQ

Once the containers are up, you can access RabbitMQ's management UI
by visiting `http://localhost:15672` (use `guest` as the username and password).
