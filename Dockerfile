FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y \
      binutils \
      upx \
      curl \
      build-essential \
      git

# Install nim
RUN curl -LO  https://nim-lang.org/choosenim/init.sh \
  && chmod +x init.sh && ./init.sh -y stable

# Install musc-gcc
RUN curl -LO https://musl.libc.org/releases/musl-1.2.1.tar.gz \
  && tar xf musl-1.2.1.tar.gz && cd musl-1.2.1 \
  && ./configure --prefix=/usr/local/musl \
  && make && make install

ADD src src
ADD config.nims .
ADD nim.cfg .
ADD memex.nimble .
RUN PATH=${PATH}:~/.nimble/bin nimble build -y
RUN upx --version && PATH=${PATH}:/usr/local/musl/bin:~/.nimble/bin nim musl -d:pcre src/memex.nim
