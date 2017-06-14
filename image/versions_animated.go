package image

import (
    "path"
    "os/exec"
    "vidalia/config"
)

func (image *Image) generateAnimatedVersions() error {
    err := image.generateMP4()
    if err != nil { return err }

    err = image.generateVP9WebM()
    if err != nil { return err }

    return image.generatePoster()
}

func (image *Image) generateMP4() error {
    destination := path.Join(image.storageDir(),
        config.AnimatedVersions["mp4"])

    args := []string {
        "-f",           "gif",
        "-i",           image.Path,
        "-crf",         "22",
        "-pix_fmt",     "yuv420p",
        "-c:v",         "libx264",
        "-preset",      "slow",
        "-movflags",    "+faststart",
        "-vf",          "scale=trunc(iw/2)*2:trunc(ih/2)*2",
        destination,
    }

    return exec.Command("ffmpeg", args...).Run()
}

func (image *Image) generateVP9WebM() error {
    destination := path.Join(image.storageDir(),
        config.AnimatedVersions["webm"])

    args := []string {
        "-f",                 "gif",
        "-i",                 image.Path,
        "-crf",               "22",
        "-pix_fmt",           "yuv420p",
        "-c:v",               "libvpx-vp9",
        "-b:v",               "1200K",
        "-tile-columns",      "6",
        "-frame-parallel",    "1",
        destination,
    }

    return exec.Command("ffmpeg", args...).Run()
}

func (image *Image) generatePoster() error {
    mp4Path := path.Join(image.storageDir(),
        config.AnimatedVersions["mp4"])
    destination := path.Join(image.storageDir(),
        config.AnimatedVersions["poster"])

    args := []string {
        "-i",       mp4Path,
        "-q:v",     "8",
        "-vframes", "1",
        destination,
    }

    return exec.Command("ffmpeg", args...).Run()
}
