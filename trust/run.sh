#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

NIC=tun0
BIN=target/release/trust

function main() {
  $1
}

function run() {
  cargo build --quiet --release && \
    sudo setcap cap_net_admin=eip "$BIN" && \
    "$BIN"
}

function setnet() {
  sudo ip addr add 10.0.0.1/24 dev "$NIC" && \
    sudo ip link set up dev "$NIC"
}

function monit() {
  sudo tshark -i "$NIC"
}

function delnet() {
  sudo ip link delete "$NIC"
}

function png() {
  ping -I $NIC 10.0.0.2
}

main "$1"
