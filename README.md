### Idea

...

### Installation

Automated: (please note that this is [deemed](https://security.stackexchange.com/questions/213401/is-curl-something-sudo-bash-a-reasonably-safe-installation-method) unsafe)

`curl -sL https://github.com/v-spassky/pexie/blob/main/infrastructure/setup.sh | sudo sh`

Manual:

...

### Usage guide

```
USAGE:
    pexie [OPTIONS]

OPTIONS:
    -h, --help                  Print help information
    -i, --ignore <ignore>...    Set vector of files/dirs relative paths that should not be included
                                into the generated HTML in the following fasion: --ignore /target  
                                /src /bin /lib
    -o, --order <order>         Set the order in which fils and dirs will appear in the generated  
                                HTML. The options are 'dirs-first'(default) and 'files-first'      
    -r, --result <result>       Set the output mode. The options are 'print'(default) and 'save'.  
                                The 'print' option tells to print the output to stdout, the 'save' 
                                option tells to save the output to pwd/pexieoutput.html
    -V, --version               Print version information
```

### Demonstration

...
