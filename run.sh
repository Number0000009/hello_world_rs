#!/bin/bash

MODEL=$1
WORKDIR=$(builtin cd $(dirname $0) && pwd)
IMAGE=${WORKDIR}/target/aarch64-unknown-none/release/hwrs

if [[ -z "$1" ]] ; then
	echo "Please provide path to the model binary"
	exit 1
fi

${MODEL} \
	-S \
	-R \
	-C cluster0.NUM_CORES=1 \
	-C cluster0.max_32bit_el=false \
	-C bp.secure_memory=false \
	-C cache_state_modelled=false \
	-C bp.pl011_uart0.uart_enable=true \
	-C bp.pl011_uart1.uart_enable=false \
	-C bp.refcounter.non_arch_start_at_default=true \
	-a cluster0.cpu0=${IMAGE}
