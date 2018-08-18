#!/bin/bash

error() {
    echo -e "\033[031m" $1 "\033[0m"
}

root=$(cd $(dirname $0); pwd)
cd $root

echo -n "Building ..."
if ! cargo build 2> /dev/null; then
    error "Failed"
    exit 1
else
    echo
fi

cd $root/testcases

for f in *.in; do
    if [ $(wc -l $f | cut -d' ' -f 1) = 0 ]; then
        continue;
    fi

    echo -n "Running \"$f\" ... "

    expected="$(cat ${f%.in}.out)"
    actually="$(cargo run < $f 2> /dev/null)"

    if [ "$expected" != "$actually" ]; then
        error "Failed."
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
