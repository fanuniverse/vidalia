FROM debian:stretch-slim

RUN apt-get update \
 && apt-get -y install curl build-essential pkg-config libmagic-dev \
libjpeg-turbo-progs libpng-dev cimg-dev \
libavformat-dev libavcodec-dev libswscale-dev ffmpeg \
libjbig0 libtiff5 libopenexr22 libpangocairo-1.0-0 \
 && curl -O https://www.imagemagick.org/download/ImageMagick.tar.gz \
 && tar xf ImageMagick.tar.gz \
 && cd ImageMagick-7.0* \
 && ./configure --enable-static=no --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick* \
 && rm -r /usr/local/share/doc/ImageMagick-7 \
 && curl -LO https://github.com/westonplatter/phashion/raw/master/ext/phashion_ext/pHash-0.9.6.tar.gz \
 && tar xf pHash-0.9.6.tar.gz \
 && cd pHash-0.9.6 \
 && ./configure CXXFLAGS="-pthread" --disable-audio-hash --disable-video-hash \
 && make \
 && make install \
 && cd .. \
 && rm -r pHash-0.9.6* \
 && ldconfig /usr/local/lib \
 && apt-get remove -y gcc make curl pkg-config \
 && apt-get autoremove -y \
 && rm -rfv /var/lib/apt/lists/*

COPY ./vidalia /usr/bin/vidalia

RUN adduser --disabled-password --gecos '' vidalia

USER vidalia

CMD vidalia
