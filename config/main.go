package config

import (
    "os"
    "path"
)

const (
    SourceFilename = "source"
)

var CacheDir = path.Join(os.Getenv("VIDALIA_PATH"), "cache")
var StorageDir = path.Join(os.Getenv("VIDALIA_PATH"), "images")
