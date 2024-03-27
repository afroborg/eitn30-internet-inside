#!/bin/bash -e

readonly TARGET=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET}/release/eitn30-internet-inside
readonly SSH_KEY=~/.ssh/eitn30-pi

readonly MAKEFILE_PATH=deploy/Makefile
readonly SERVICE_NAME=longge.service

# Provided by tailscale
readonly BASE_IP="100.124.31.24"
readonly MOBILE_IP="100.65.157.26"

while getopts "n:" opt; do
  case ${opt} in
    n )
      PI_NUMBER=${OPTARG}
      if [ ${OPTARG} = "06" ]
      then
        PI_IP=${BASE_IP}
      elif [ ${OPTARG} = "24" ]
      then
        PI_IP=${MOBILE_IP}
      else
        echo "Invalid PI_NUMBER"
        exit 1
      fi
      ;;
    \? )
      echo "Usage: deploy.sh [-n <pi-ip>]"
      exit 1
      ;;
  esac
done

echo "Deploying to inuti${PI_NUMBER}"
rsync ${BINARY_PATH} ${MAKEFILE_PATH} -e "ssh -i ${SSH_KEY}" pi@${PI_IP}:~/eitn30 > /dev/null 2>&1

echo "Restarting service"
ssh -i "${SSH_KEY}" -A pi@"${PI_IP}" "sudo systemctl restart '${SERVICE_NAME}'" > /dev/null 2>&1

echo "Done"
