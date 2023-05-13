#!/bin/bash


find src | entr -r cargo clippy -- -W clippy::pedantic -W clippy::nursery

## Adds --fix that attempts to fix stuff. Should commit before running it, because it can change code
# find src | entr -r cargo clippy --fix -- -W clippy::pedantic -W clippy::nursery

## Even more strict pre-production checks to force propper handling of results instead of unwrap/expect "cop-outs"
# find src | entr -r cargo clippy --fix -W clippy::pedantic -W clippy::nursery -W cluppy::unwrap_used -W clippy::expect_used