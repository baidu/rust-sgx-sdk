source /opt/sgxsdk/environment && \
cd /root && \
git clone --recursive https://github.com/intel/linux-sgx && \
cd linux-sgx && \
git checkout sgx_2.14 && \
./download_prebuilt.sh && \
make deb_local_repo && \
cd linux/installer/deb && \
find . -maxdepth 2 -name '*.deb' | grep -v pccs | grep -v sgx-ra-service | xargs dpkg -i && \
mkdir /var/run/aesmd && \
cd /root && rm -rf /root/linux-sgx
