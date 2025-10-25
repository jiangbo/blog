#!/usr/bin/env bash

cd blog || exit
for month in 2*/*; do
    number=100
    for day in "$month"/*.md; do
        number=$((number + 1))
        name=$(basename "$day" .md)
        if [[ "$name" == *"】"* ]]; then
            tag=${name%】*}
            tag=${tag:1}
        fi
        if [[ "$name" == *"："* ]]; then
            tag=${name%：*}
        fi
        var="+++\ntitle = '${name}'\ndate = ${day:0:4}-${day:9:2}-${number:1}\n\n[taxonomies]\ncategories = ['${tag}']\n+++\n"
        sed -i "1s/.*/$var/" "$day"
    done
done