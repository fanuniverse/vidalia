FROM debian:jessie-backports

RUN apt-get update \
 && apt-get -y install git curl build-essential pkg-config \
libjpeg-turbo-progs libpng-dev cimg-dev \
libavformat-dev libavcodec-dev libswscale-dev ffmpeg \
libmagic-dev \
 && rm -rfv /var/lib/apt/lists/*

RUN curl -O https://www.imagemagick.org/download/ImageMagick.tar.gz \
 && tar xf ImageMagick.tar.gz \
 && cd ImageMagick* \
 && ./configure \
 && make \
 && make install \
 && cd .. \
 && rm -rv ImageMagick*

RUN curl -LO https://github.com/westonplatter/phashion/raw/master/ext/phashion_ext/pHash-0.9.6.tar.gz \
 && tar xf pHash-0.9.6.tar.gz \
 && cd pHash-0.9.6 \
 && ./configure CXXFLAGS="-pthread" --disable-audio-hash --disable-video-hash \
 && make \
 && make install \
 && cd .. \
 && rm -rv pHash-0.9.6* \
 && ldconfig /usr/local/lib

COPY ./vidalia /usr/bin/vidalia

RUN adduser --disabled-password --gecos '' vidalia

USER vidalia

CMD vidalia
