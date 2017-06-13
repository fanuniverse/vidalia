package main

import (
    "fmt"
    "vidalia/image"
)

func main() {
    img := image.NewImage("test/images/small.jpg")
    err := img.Process()

    if err != nil {
        panic(err)
    } else {
        fmt.Println(img)
    }
}
