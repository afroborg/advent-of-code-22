#!/bin/bash

echo "Creating new advent of code day"
echo "What day should it be for?"

read day

echo "Creating a folder for day $day"

cp -r day-template ./day-$day
sed -i '' "s/day-template/day-$day/g" "./day-$day/Cargo.toml"
