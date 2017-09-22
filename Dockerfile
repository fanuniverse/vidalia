# docker build --target dev --build-arg release=0

FROM rust:1.19.0-stretch AS dev

RUN apt-get update \
 && apt-get -y install git curl build-essential clang pkg-config \
libjpeg-turbo-progs libpng-dev cimg-dev \
libavformat-dev libavcodec-dev libavfilter-dev libswscale-dev ffmpeg \
libmagic-dev \
 && rm -rfv /var/lib/apt/lists/*

ENV MAGICK_VERSION 7.0.7-3

RUN curl https://www.imagemagick.org/download/ImageMagick-${MAGICK_VERSION}.tar.gz | tar xz \
 && cd ImageMagick-${MAGICK_VERSION} \
 && ./configure --with-magick-plus-plus=no --with-perl=no \
 && make \
 && make install \
 && cd .. \
 && rm -r ImageMagick-${MAGICK_VERSION}

RUN ldconfig

WORKDIR /usr/src/vidalia

# docker build --target dev

ARG release=1

COPY . /usr/src/vidalia

RUN adduser --disabled-password --gecos '' vidalia; chown -R vidalia .

USER vidalia

ENV USER vidalia

RUN bash -c '[[ ${release} -eq "0" ]] || cargo build --release'

# docker build

FROM debian:stretch-slim AS release

RUN apt-get update \
 && apt-get -y install libjpeg-turbo-progs libpng-tools \
libjbig0 liblcms2-2 libtiff5 libopenexr22 liblqr-1-0 \
libxt6 libgvc6 libwmf0.2-7 libpangocairo-1.0-0 libdjvulibre21 \
ffmpeg \
 && rm -rfv /var/lib/apt/lists/*

COPY --from=0 /usr/local/lib/libMagickWand-7.Q16HDRI.so.4 /usr/local/lib
COPY --from=0 /usr/local/lib/libMagickCore-7.Q16HDRI.so.4 /usr/local/lib
COPY --from=0 /usr/src/vidalia/target/release/vidalia /usr/local/bin

ENV LD_LIBRARY_PATH=/usr/local/lib
RUN ldconfig

USER nobody

CMD vidalia
