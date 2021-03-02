# fdns-filter

A small lib/cli to extract information from [Rapid7's FDNS datasets](https://opendata.rapid7.com/sonar.fdns_v2/)

Using the CLI:

```bash
USAGE:
    fdns-filter [FLAGS] [OPTIONS] --kind <kind> --path <path> --regex <regex>

FLAGS:
    -h, --help       Prints help information
    -v, --value      filter on value field (if omitted it automatically uses the name field)
    -V, --version    Prints version information

OPTIONS:
    -a, --allow-list <allow-list>    path to txt containing allowed domains [env: FDNS_ALLOW_LIST=]
    -k, --kind <kind>                which kind to look for A, AAAA, TXT, MX or CNAME [env: FDNS_KIND=]
    -o, --output <output>            table or json output [env: FDNS_OUTPUT=]  [default: table]
    -p, --path <path>                path to fdns gzip file [env: FDNS_PATH=]
    -r, --regex <regex>              regex pattern to use as filter [env: FDNS_REGEX=]
```
