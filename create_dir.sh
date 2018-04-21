#!/bin/bash

set -eu

root=$(cd $(dirname $0); pwd)

mkdir $1
cd $_

create_dir() {
    cargo new --bin --vcs none $1

    cat >> $1/Cargo.toml << EOF

[features]
debug = []
EOF

    cp $root/Templates/template.rs $1/src/main.rs
}

shift

if [ $# -eq 0 ]; then
    create_dir b
    create_dir c
    create_dir d
else
    for d in $@; do
        create_dir $d
    done
fi
