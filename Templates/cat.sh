#!/bin/bash

set -eu

root=$(cd $(dirname $0); pwd)
cd $root

sed "/^use lib::*/r src/lib.rs" src/bin/$1.rs | sed "/^use lib::*/d"
