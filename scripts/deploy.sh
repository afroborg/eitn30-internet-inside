#!/bin/bash -e

readonly TARGET=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET}/release/eitn30-internet-inside
readonly SSH_KEY=~/.ssh/eitn30-pi

readonly MAKEFILE_PATH=deploy/Makefile
readonly PERFORMANCE_SCRIPT_PATH=performance/performance.py
readonly PERFORMANCE_REQUIREMENTS_PATH=performance/requirements.txt
readonly SERVICE_NAME=longge.service

# Provided by tailscale
readonly BASE_IP="100.93.60.11"
readonly MOBILE_IP="100.65.157.26"

while getopts "n:" opt; do
  case ${opt} in
    n )
      PI_NUMBER=${OPTARG}
      if [ ${OPTARG} = "32" ]
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

FILES="${BINARY_PATH} ${MAKEFILE_PATH}"

if [ ${PI_IP} = ${MOBILE_IP} ]
then
  FILES="${FILES} ${PERFORMANCE_SCRIPT_PATH} ${PERFORMANCE_REQUIREMENTS_PATH}"
fi

rsync -r ${FILES} -e "ssh -i ${SSH_KEY}" pi@${PI_IP}:~/eitn30 > /dev/null

echo "Restarting service"
ssh -i "${SSH_KEY}" -A pi@"${PI_IP}" "sudo systemctl restart '${SERVICE_NAME}'" > /dev/null

echo "Done"
