#!/bin/bash

set -u

root=$(cd $(dirname $0); pwd)
cd $root

function err {
    echo -e "\033[031m$1\033[0m"
}

function ok {
    echo -e "\033[032m$1\033[0m"
}

function run { (
    task=$1
    cd $task

    echo -n "Building $task ... "
    if ! cargo build --bin $task 2> /dev/null; then
        err "Failed"
        exit
    else
        ok "Done"
    fi

    for input in *.in; do
        if [ $(wc -l $input | cut -d' ' -f 1) -eq 0 ]; then
            continue;
        fi

        echo -n "Running against \"$task/$input\" ... "

        expected="$(cat ${input%.in}.out)"
        actually="$(cargo run --bin $task < $input 2> /dev/null)"

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
) }

if [ ! -e $root/testcases ]; then
    err "No testcases dir"
    exit 1
fi
cd $root/testcases

if [ $# -eq 0 ]; then
    for d in *; do
        run $d
    done
else
    for d in $@; do
        run $d
    done
fi
