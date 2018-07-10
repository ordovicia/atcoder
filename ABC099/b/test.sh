#!/bin/bash

echo_red() {
    echo -e "\033[031m" $1 "\033[0m"
}

root=$(cd $(dirname $0); pwd)
cd $root/tests

echo -n "Building ..."
if ! cargo build 2> /dev/null; then
    echo_red "Failed"
    exit 1
else
    echo
fi

for f in *.in; do
    echo -n "Running \"$f\" ... "

    expected="$(cat ${f%.in}.out)"
    actually="$(cargo run < $f 2> /dev/null)"

    if [ "$expected" != "$actually" ]; then
        echo_red "Failed."
        cat << EOF
Expected:
$expected

Actually:
$actually

EOF
    else
        echo
    fi
done
