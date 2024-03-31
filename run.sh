#! /usr/bin/bash

for x in $@
do
    rustc $x.rs
    ./$x
    rm $x
done