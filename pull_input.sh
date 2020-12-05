#!/bin/sh

if [ -z $1 ]; then
  echo "Provide day"
  exit 1
fi

DAY=$1

BASEDIR=$(dirname $0)

URL="https://adventofcode.com/2020/day/$DAY/input"

HEADER="cookie: $(cat $BASEDIR/header)"

curl -H "$HEADER" "$URL" > input.txt
