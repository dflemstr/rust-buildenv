#!/bin/sh -eux

. /etc/lsb-release

apt-get update

apt-get install -y \
        software-properties-common python-software-properties curl wget sudo g++-multilib libglib2.0-dev git subversion
add-apt-repository ppa:ubuntu-toolchain-r/test
apt-get remove -y --purge \
        software-properties-common python-software-properties

curl -sSLf http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
echo "deb http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME main" >> /etc/apt/sources.list.d/llvm.list
echo "deb-src http://apt.llvm.org/$DISTRIB_CODENAME/ llvm-toolchain-$DISTRIB_CODENAME main" >> /etc/apt/sources.list.d/llvm.list

apt-get update
apt-get install -y \
        clang-5.0 libclang-5.0-dev libclang1-5.0 lld-5.0

apt-get autoremove -y
rm -rf /var/lib/apt/lists/*
