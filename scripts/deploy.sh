# /bin/bash -e

readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET_ARCH}/release/eitn30-internet-inside
readonly SSH_KEY=~/.ssh/eitn30-pi

while getopts "n:" opt; do
  case ${opt} in
    n )
      PI_IP=inputi${OPTARG}.lab.eit.lth.se
      ;;
    \? )
      echo "Usage: deploy.sh [-n <pi-ip>]"
      exit 1
      ;;
  esac
done

echo "Deployed to ${PI_IP}"
rsync ${BINARY_PATH} -e "ssh -i ${SSH_KEY}" pi@${PI_IP}:~/eitn30 >/dev/null 2>&1
echo "Done"
