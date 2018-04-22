#!/bin/sh -eux

. /etc/lsb-release

apt-get update

apt-get install -y \
        software-properties-common python-software-properties curl wget sudo g++-multilib libglib2.0-dev git subversion bzip2
add-apt-repository ppa:ubuntu-toolchain-r/test
apt-get remove -y --purge \
        software-properties-common python-software-properties

curl -sSLf http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
echo "deb http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME main" >> /etc/apt/sources.list.d/llvm.list
echo "deb-src http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME main" >> /etc/apt/sources.list.d/llvm.list

apt-get update
apt-get install -y \
        clang-6.0 libclang-6.0-dev libclang1-6.0 llvm-6.0-dev lld-6.0

apt-get autoremove -y
rm -rf /var/lib/apt/lists/*
