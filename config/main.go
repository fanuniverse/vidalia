package config

import (
    "os"
    "path"
)

var AmqpUri = os.Getenv("AMQP_URI")

/* We receive files that are located in the cache directory
 * and move them to storage, replacing the temporary file name
 * with SourceFilename and setting an extension that is appropriate
 * for the MIME type. */
var CacheDir = path.Join(os.Getenv("VIDALIA_PATH"), "cache")
var StorageDir = path.Join(os.Getenv("VIDALIA_PATH"), "images")
const SourceFilename = "source"

/* If the MIME type is not found in this map, the file is not
 * moved to storage; instead, an error is reported. */
var AllowedExtensions = map[string]string {
    "image/png": "png",
    "image/jpeg": "jpg",
    "image/gif": "gif",
}

/* Then we proceed to generate image versions (thumbnails)
 * from this map (file name => width). */
var ImageVersions = map[string]uint {
    "thumbnail": 300,
    "preview": 1280,
}

/* For GIF images, we generate an MP4/WebM render and
 * a static poster (very small in size, used as a placeholder
 * while the video is loading) */
var AnimatedVersions = map[string]string {
    "mp4": "rendered.mp4",
    "webm": "rendered.webm",
    "poster": "poster.jpg",
}
