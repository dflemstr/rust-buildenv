#!/bin/sh -eux

. /etc/lsb-release

apt-get update

apt-get install -y \
        software-properties-common python-software-properties curl wget sudo g++-multilib libglib2.0-dev git
add-apt-repository ppa:ubuntu-toolchain-r/test
apt-get remove -y --purge \
        software-properties-common python-software-properties
apt-get autoremove -y

curl -sSLf http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
echo "deb http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME-4.0 main" >> /etc/apt/sources.list.d/llvm.list
echo "deb-src http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME-4.0 main" >> /etc/apt/sources.list.d/llvm.list

apt-get update
apt-get install -y \
        clang-4.0 libclang-4.0-dev libclang1-4.0

rm -rf /var/lib/apt/lists/*
