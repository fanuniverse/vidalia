package image

import (
    "os"
    "fmt"
    "path"
    "vidalia/config"
)

func (image *Image) storageDir() string {
    return path.Join(config.StorageDir, fmt.Sprint(image.id))
}

func (image *Image) versionStoragePath(version string) string {
    return path.Join(image.storageDir(),
        fmt.Sprintf("%s.%s", version, image.ext))
}

func (image *Image) moveCachedToStorage() (err error) {
    err = os.Mkdir(image.storageDir(), 0755)
    if err != nil { return err }

    storagePath := path.Join(image.storageDir(),
        fmt.Sprintf("%s.%s", config.SourceFilename, image.ext))

    err = os.Rename(image.path, storagePath)
    if err != nil { return err }

    image.path = storagePath
    return nil
}

func (image *Image) linkVersionToImage(version string) error {
    return os.Link(image.path, image.versionStoragePath(version))
}
