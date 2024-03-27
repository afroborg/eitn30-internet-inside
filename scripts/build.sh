#!/bin/bash -e

readonly TARGET=aarch64-unknown-linux-gnu

cross build --release --target ${TARGET}