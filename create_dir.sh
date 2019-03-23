#!/bin/bash

set -eu

root=$(cd $(dirname $0); pwd)
template="$root/Templates"
cd $root

d=$1
shift

cargo new --lib --vcs none "$d" --name lib
cat >> "$d/Cargo.toml" << EOF

[features]
debug = []
EOF

cp $template/lib.rs $d/src/lib.rs
cp $template/{test,cat}.sh $d/

cp_main() {
    mkdir $d/src/bin
    for t in $@; do
        cp $template/main.rs $d/src/bin/$t.rs
    done
}

if [ $# -eq 0 ]; then
    cp_main a b c d
else
    cp_main $@
fi
