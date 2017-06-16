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

RUN curl -O https://storage.googleapis.com/golang/go1.8.3.linux-amd64.tar.gz \
 && tar xf go1.8.3.linux-amd64.tar.gz \
 && rm go1.8.3.linux-amd64.tar.gz \
 && mv go /usr/local \
 && echo 'export GOPATH=/go' >> ~/.bashrc \
 && echo 'export PATH=$PATH:/usr/local/go/bin:$GOPATH/bin' >> ~/.bashrc

RUN curl -LO https://github.com/westonplatter/phashion/raw/master/ext/phashion_ext/pHash-0.9.6.tar.gz \
 && tar xf pHash-0.9.6.tar.gz \
 && cd pHash-0.9.6 \
 && ./configure CXXFLAGS="-pthread" --disable-audio-hash --disable-video-hash \
 && make \
 && make install \
 && cd .. \
 && rm -rv pHash-0.9.6* \
 && ldconfig /usr/local/lib

RUN bash -c "source ~/.bashrc; \
go get gopkg.in/gographics/imagick.v3/imagick; \
go get github.com/kavu/go-phash; \
go get github.com/rakyll/magicmime; \
go get github.com/stretchr/testify; \
go get github.com/buger/jsonparser; \
go get github.com/streadway/amqp" 
