#!/bin/bash

set -eu

root=$(cd $(dirname $0); pwd)

if [ ! -e $1 ]; then
    mkdir $1
fi
cd $1

create_dir() {
    for d in $@; do
        cargo new --bin --vcs none $d
        cat >> $d/Cargo.toml << EOF

[features]
debug = []
EOF

        cp $root/Templates/template.rs $d/src/main.rs
        cp $root/Templates/test.sh $d/
    done
}

shift

if [ $# -eq 0 ]; then
    create_dir a b c d
else
    create_dir $@
fi
