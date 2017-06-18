package test

import (
    "path"
    "testing"
    "github.com/stretchr/testify/assert"
    "vidalia/config"
)

func TestLargeImage(t *testing.T) {
    img := processOverAmqp(t, "large.jpg", "25")
    assertStored(t, "large.jpg", "25", img)
    defer cleanUpStored("large.jpg", "25", img)

    /* Analysis test */
    assert.Equal(t, uint(1920), img.Width)
    assert.Equal(t, uint(1080), img.Height)
    assert.Equal(t, "1111001100011111110100010010000000000111011111001100110000110011", img.Phash)

    /* Version test */
    previewW, previewH := imageInfo(t,
        path.Join(config.StorageDir, "25/preview.jpg"))
    assert.Equal(t, uint(1280), previewW)
    assert.Equal(t, uint(720), previewH)

    thumbW, thumbH := imageInfo(t,
        path.Join(config.StorageDir, "25/thumbnail.jpg"))
    assert.Equal(t, uint(300), thumbW)
    assert.Equal(t, uint(168), thumbH)
}
