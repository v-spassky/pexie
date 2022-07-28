#!/bin/bash

# This script handles local setup of the pexie script for development and testing.

rm -r $HOME/pexie
mkdir $HOME/pexie
cp binary/pexie $HOME/pexie
cp templates/entrypoint_template.html $HOME/pexie
cp templates/recuresively_included_template.html $HOME/pexie
