package test

import (
    "os"
    "path"
    "testing"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
    "vidalia/image"
    "vidalia/config"
)

func TestLargeImage(t *testing.T) {
    cached := path.Join(config.CacheDir, "large.jpg")
    targetDir := path.Join(config.StorageDir, "24")
    target := path.Join(targetDir, "source.jpg")

    img, err := image.NewImage("large.jpg", "24")
    require.Nil(t, err)

    require.Nil(t, err)
    assert.Equal(t, target, img.Path,
        "image.Path should be updated with the storage location.")

    if _, err := os.Stat(cached); os.IsExist(err) {
        assert.Fail(t, "Cached file should be destroyed (moved)")
    }
    if _, err := os.Stat(target); os.IsNotExist(err) {
        assert.Fail(t, "Target file should exist")
    }

    err = img.Process()
    require.Nil(t, err)

    /* Reset test image location for subsequent tests */
    defer func() {
        os.Rename(target, cached)
        os.RemoveAll(targetDir)
    }()

    /* Analysis test */
    assert.Equal(t, uint(1920), img.Width)
    assert.Equal(t, uint(1080), img.Height)
    assert.Equal(t, uint64(17518951010989296691), img.Phash)

    /* Version test */
    previewW, previewH := imageInfo(t,
        path.Join(config.StorageDir, "24/preview.jpg"))
    assert.Equal(t, uint(1280), previewW)
    assert.Equal(t, uint(720), previewH)

    thumbW, thumbH := imageInfo(t,
        path.Join(config.StorageDir, "24/thumbnail.jpg"))
    assert.Equal(t, uint(300), thumbW)
    assert.Equal(t, uint(168), thumbH)
}
