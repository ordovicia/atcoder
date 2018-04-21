#!/bin/bash

color_green() {
    echo -en "\033[032m"
}

color_red() {
    echo -en "\033[031m"
}

color_reset() {
    echo -en "\033[0m"
}

for f in *.in; do
    color_green
    echo -n "Running test \"$f\" ... "

    expected="$(cat ${f%.in}.out)"
    actually="$(cargo run < $f 2> /dev/null)"

    if [ $expected != $actually ]; then
        color_red
        echo "Failed."
        color_reset

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

color_reset
