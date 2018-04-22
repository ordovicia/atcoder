#!/bin/bash

root=$(cd $(dirname $0); pwd)
cd $root/tests

echo "Building ..."

for f in *.in; do
    echo -n "Running \"$f\" ... "

    expected="$(cat ${f%.in}.out)"
    actually="$(cargo run < $f 2> /dev/null)"

    if [ $expected != $actually ]; then
        echo -e "\033[031mFailed.\033[0m"
        cat << EOF
Expected:
$expected

Actually:
$actually

EOF
    else
        echo ""
    fi
done
