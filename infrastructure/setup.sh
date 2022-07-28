#!/bin/bash

# This script manages installation and setup of the pexie tool.
# More info: https://github.com/v-spassky/pexie

# Make a directory to store everything needed
mkdir $HOME/pexie

# Download everything needed
sudo wget https://github.com/v-spassky/pexie/blob/main/binary/pexie -P $HOME/pexie
sudo wget https://github.com/v-spassky/pexie/blob/main/templates/entrypoint_template.html -P $HOME/pexie
sudo wget https://github.com/v-spassky/pexie/blob/main/templates/recuresively_included_template.html -P $HOME/pexie

# Set up an alias for the current session
alias pexie='~/pexie/pexie'

# Make the alias persistent
echo '' >> $HOME/.bashrc
echo '# Set up a shortcut for pexie script' >> $HOME/.bashrc
echo "alias pexie='~/pexie/pexie'" >> $HOME/.bashrc
