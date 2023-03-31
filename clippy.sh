#!/bin/sh

cargo clippy --fix -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used
