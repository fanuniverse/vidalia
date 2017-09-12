FROM rust:1.19.0-stretch

RUN apt-get update \
 && apt-get -y install git curl build-essential pkg-config \
libjpeg-turbo-progs libpng-dev cimg-dev \
libavformat-dev libavcodec-dev libswscale-dev ffmpeg \
libmagic-dev
# && rm -rfv /var/lib/apt/lists/*

RUN curl -O https://www.imagemagick.org/download/ImageMagick.tar.gz \
 && tar xf ImageMagick.tar.gz \
 && cd ImageMagick-7.0* \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-7.0*

RUN apt-get install -y clang libavfilter-dev

RUN adduser --disabled-password --gecos '' vidalia

USER vidalia

ENV USER vidalia

WORKDIR /usr/src/vidalia
