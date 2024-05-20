#!/bin/bash -e

# Constants
readonly TARGET=aarch64-unknown-linux-gnu
readonly BINARY_PATH=target/${TARGET}/release/eitn30-internet-inside
readonly SSH_KEY=~/.ssh/eitn30-pi

readonly MAKEFILE_PATH=deploy/Makefile
readonly PERFORMANCE_SCRIPT_PATH=performance/performance.py
readonly PERFORMANCE_REQUIREMENTS_PATH=performance/requirements.txt
readonly SERVICE_NAME=longge.service

readonly PI_NUMBER=$1
readonly PI_IP=$2

echo "Deploying to inuti${PI_NUMBER}"

FILES="${BINARY_PATH} ${MAKEFILE_PATH}"

if [ $3 = "--mobile" ]
then
  echo "Deploying mobile"
  FILES="${FILES} ${PERFORMANCE_SCRIPT_PATH} ${PERFORMANCE_REQUIREMENTS_PATH}"

  else
  echo "Deploying base"
fi

rsync -r ${FILES} -e "ssh -i ${SSH_KEY}" pi@${PI_IP}:~/eitn30 > /dev/null

echo "Restarting service"
ssh -i "${SSH_KEY}" -A pi@"${PI_IP}" "sudo systemctl restart '${SERVICE_NAME}'" > /dev/null

echo "Done"
