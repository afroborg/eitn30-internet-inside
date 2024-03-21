# /bin/bash -e

readonly TARGET_ARCH=aarch64-unknown-linux-gnu

cross build --target ${TARGET_ARCH} --release