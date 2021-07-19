if [ $BINUTILS_DIST != "SELF_BUILT" ]
then
    cd /root && \
    wget https://download.01.org/intel-sgx/sgx-linux/2.14/as.ld.objdump.gold.r3.tar.gz && \
    tar xzf as.ld.objdump.gold.r3.tar.gz && \
    cp -r external/toolset/$BINUTILS_DIST/* /usr/bin/ && \
    rm -rf ./external ./as.ld.objdump.gold.r3.tar.gz
else
    curl -o binutils.tar.xz https://ftp.gnu.org/gnu/binutils/binutils-2.35.tar.xz && \
    tar xf binutils.tar.xz && \
    cd binutils-2.35 && \
    mkdir build && \
    cd build && \
    ../configure --prefix=/usr/local --enable-gold --enable-ld=default --enable-plugins --enable-shared --disable-werror --enable-64-bit-bfd --with-system-zlib && \
    make -j "$(nproc)" && \
    make install && \
    cd /root && \
    rm -rf binutils-gdb
fi
