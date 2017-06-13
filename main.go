package main

import (
    "fmt"
    "path"
    "vidalia/image"
    "vidalia/config"
)

func main() {
    cachePath := path.Join(config.CacheDir, "r4nd0mc4ch357r1n6.jpg")
    var imgId uint = 23

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
