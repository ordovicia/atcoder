#!/bin/bash

set -u

function err {
    echo -e "\033[031m$1\033[0m"
}

function ok {
    echo -e "\033[032m$1\033[0m"
}

root=$(cd $(dirname $0); pwd)
cd $root

if [ ! -e $root/testcases ]; then
    err "No testcases dir"
    exit 1
fi
cd $root/testcases

for d in *; do (
    cd $d

    echo -n "Building $d ... "
    if ! cargo build --bin $d 2> /dev/null; then
        err "Failed"
        exit
    else
        ok "Done"
    fi

    for f in *.in; do
        if [ $(wc -l $f | cut -d' ' -f 1) -eq 0 ]; then
            continue;
        fi

        echo -n "Running against \"$d/$f\" ... "

        expected="$(cat ${f%.in}.out)"
        actually="$(cargo run --bin $d < $f 2> /dev/null)"

        if [ "$expected" != "$actually" ]; then
            err "Failed"
            cat << EOF
    Expected:
    $expected

    Actually:
    $actually

EOF
        else
            ok "Passed"
        fi
    done
    echo
) done
