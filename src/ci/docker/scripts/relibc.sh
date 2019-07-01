set -ex

hide_output() {
  set +x
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  $@ &> /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  rm /tmp/build.log
  set -x
}

mkdir -p /relibc

#git clone -b relibc https://gitlab.redox-os.org/mati865/libc.git
#cd libc
#git submodule update --init --recursive
#PREFIX=/relibc ./setup.sh all

# Build with https://gitlab.redox-os.org/mati865/libc/tree/relibc
curl -LO https://github.com/mati865/llvm-project/releases/download/llvmorg-8.0.0-relibc_v2/prebuilt.zip
unzip prebuilt.zip -d /relibc

ln -s /relibc/bin/x86_64-unknown-linux-relibc-clang{,++} /usr/local/bin/

#for lib in dl rt util; do
#    /relibc/bin/llvm-ar rc "/relibc/lib/lib${lib}.a"
#done
#cp /relibc/lib/{ld64.so.1,libc.so} /lib/
