package test

import (
    "path"
    "testing"
    "github.com/stretchr/testify/assert"
    "vidalia/config"
)

func TestAnimatedImage(t *testing.T) {
    img := processOverAmqp(t, "animated.gif", "24")
    assertStored(t, "animated.gif", "24", img)
    defer cleanUpStored("animated.gif", "24", img)

    /* Analysis test */
    assert.Equal(t, uint(500), img.Width)
    assert.Equal(t, uint(280), img.Height)
    assert.Equal(t, "111001110011100111001010110001110010000100011101000101110110100", img.Phash)

    /* Version test */
    webmW, webmH, codec := videoInfo(t,
        path.Join(config.StorageDir, "24/rendered.webm"))
    assert.Equal(t, "vp9", codec)
    assert.Equal(t, int64(500), webmW)
    assert.Equal(t, int64(280), webmH)

    mp4W, mp4H, codec := videoInfo(t,
        path.Join(config.StorageDir, "24/rendered.mp4"))
    assert.Equal(t, "h264", codec)
    assert.Equal(t, int64(500), mp4W)
    assert.Equal(t, int64(280), mp4H)

    posterW, posterH := imageInfo(t,
        path.Join(config.StorageDir, "24/poster.jpg"))
    assert.Equal(t, uint(500), posterW)
    assert.Equal(t, uint(280), posterH)
}
