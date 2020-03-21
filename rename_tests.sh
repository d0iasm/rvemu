#!/bin/sh

OBJCOPY=${RISCV}/bin/riscv64-unknown-elf-objcopy
ISA=${RISCV}/target/share/riscv-tests/isa/

IN_DIR=tests/resources/original
OUT_DIR=tests/resources

for file in ${IN_DIR}/*
do
    original=$(basename $file)
    filename=$(echo $original | sed -e "s/-/_/g")
    echo ${filename}
    ${OBJCOPY} -O binary ${file} ${OUT_DIR}/${filename}
    #cp ${file} ${OUT_DIR}/${filename}
done
