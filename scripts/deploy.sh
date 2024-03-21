# /bin/bash -e

readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET_ARCH}/release/$(basename $(pwd))
readonly PI_IP=inuti06.lab.eit.lth.se

rsync ${BINARY_PATH} -e "ssh -i ~/.ssh/eitn30-pi" pi@${PI_IP}:~/eitn30
echo "Deployed to ${PI_IP}"