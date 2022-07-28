#!/bin/bash

# This script runs release binary compilation and saves it to the /binary folder.

cargo build --release
rm binary/pexie
cp target/release/pexie binary
