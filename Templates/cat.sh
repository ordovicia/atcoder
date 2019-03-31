#!/bin/bash

set -eu

root=$(cd $(dirname $0); pwd)
cd $root

tmp=$(mktemp)
sed "/^use/d" src/lib.rs > $tmp
sed "/^use lib::*/r $tmp" src/bin/$1.rs | sed "/^use lib::*/d"
