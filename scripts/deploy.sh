# /bin/bash -e

readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET_ARCH}/release/eitn30-internet-inside
readonly PI_IP=inuti06.lab.eit.lth.se
readonly SSH_KEY=~/.ssh/eitn30-pi

rsync ${BINARY_PATH} -e "ssh -i ${SSH_KEY}" pi@${PI_IP}:~/eitn30 >/dev/null 2>&1
echo "Deployed to ${PI_IP}"