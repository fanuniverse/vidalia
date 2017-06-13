package image

import (
    "fmt"
    "errors"
    "github.com/rakyll/magicmime"
)

/* Quick check to validate file validity and
 * ensure consistent extensions (no .jpeg and .jpg) */
var AllowedExtensions = map[string]string {
    "image/png": "png",
    "image/jpeg": "jpg",
    "image/gif": "gif",
}

func validImageExtension(path string) (ext string, err error) {
    mime, err := imageMime(path)
    if err != nil { return "", err }

    if ext, ok := AllowedExtensions[mime]; ok {
        return ext, nil
    } else {
        return "", errors.New(fmt.Sprintf("%s files are not allowed", mime))
    }
}

func imageMime(path string) (mime string, err error) {
    err = magicmime.Open(
            magicmime.MAGIC_MIME_TYPE |
            magicmime.MAGIC_ERROR |
            magicmime.MAGIC_NO_CHECK_COMPRESS |
            magicmime.MAGIC_NO_CHECK_CDF |
            magicmime.MAGIC_NO_CHECK_ELF |
            magicmime.MAGIC_NO_CHECK_TAR |
            magicmime.MAGIC_NO_CHECK_TEXT |
            magicmime.MAGIC_NO_CHECK_TOKENS |
            magicmime.MAGIC_NO_CHECK_ENCODING)
    if err != nil { return "", err }
    defer magicmime.Close()

    return magicmime.TypeByFile(path)
}
