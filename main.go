package main

import (
    "fmt"
    "path"
    "vidalia/image"
    "vidalia/config"
)

func main() {
    cachePath := path.Join(config.CacheDir, "small.jpg")
    var imgId uint = 24

    img, err := image.NewImage(cachePath, imgId)

    if err != nil {
        panic(err)
    }

    err = img.Process()

    if err != nil {
        panic(err)
    } else {
        fmt.Println(img)
    }
}
