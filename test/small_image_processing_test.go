package test

import (
    "os"
    "path"
    "testing"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
    "vidalia/config"
)

func TestSmallImage(t *testing.T) {
    img := processOverAmqp(t, "small.jpg", "26")
    assertStored(t, "small.jpg", "26", img)
    defer cleanUpStored("small.jpg", "26", img)

    /* Analysis test */
    assert.Equal(t, uint(200), img.Width)
    assert.Equal(t, uint(198), img.Height)
    assert.Equal(t, "1011110010110011111010001010100110101000000000111110101001001011", img.Phash)

    /* Version test */
    source, err := os.Stat(
        path.Join(config.StorageDir, "26/source.jpg"))
    require.Nil(t, err)

    preview, err := os.Stat(
        path.Join(config.StorageDir, "26/preview.jpg"))
    require.Nil(t, err)

    thumb, err := os.Stat(
        path.Join(config.StorageDir, "26/thumbnail.jpg"))
    require.Nil(t, err)

    assert.True(t, os.SameFile(preview, source),
        "preview should be hardlinked to the source")

    assert.True(t, os.SameFile(thumb, source),
        "thumbnail should be hardlinked to the source")
}
