package main

import (
    "fmt"
    "vidalia/image"
)

func main() {
    img, err := image.NewImage("test/images/small.jpg")

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
