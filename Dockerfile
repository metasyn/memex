FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y \
      binutils \
      curl \
      build-essential \
      git

# Install nim
RUN curl -LO  https://nim-lang.org/choosenim/init.sh \
    && chmod +x init.sh && ./init.sh -y stable && mv /root/.nimble/bin/* /usr/local/bin \
    # Install musc-gcc
    && curl -LO https://musl.libc.org/releases/musl-1.2.1.tar.gz \
    && tar xf musl-1.2.1.tar.gz && cd musl-1.2.1 \
    && ./configure --prefix=/usr/local/musl \
    && make && make install \
    && mv /usr/local/musl/bin/musl-gcc /usr/local/bin && \
    # Intall upx
    curl -L -o upx.tar.xz https://github.com/upx/upx/releases/download/v3.96/upx-3.96-i386_linux.tar.xz \
    && tar xf upx.tar.xz && cd upx-3.96-i386_linux && chmod +x upx && mv upx /usr/local/bin

ADD src src
ADD config.nims .
ADD nim.cfg .
ADD memex.nimble .
RUN nimble install -y && nim musl -d:pcre src/memex.nim
