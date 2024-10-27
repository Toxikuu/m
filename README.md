# mo

## info
mo is a utility that mimics basic mv functionality. it's significantly more dangerous than mv as it will overwrite non-empty directories without flinching. mo was created as a utility for rid-meta.

mo exceeds at overwriting, making it useful for install scripts.

## installation
mo comes with rid, but it can also be used as a standalone binary. the latest release should have an mo binary attached. download it and move it to /bin

## usage
mo takes two arguments - src and dst. mo accepts no flags. mo makes backups in /tmp/m. basic usage information and a warning are displayed if mo is used improperly.

### ex.
```bash
# mv would move openssl into the openssl-3.4.0 directory if it existed
# mo overwrites it
mo /usr/share/doc/openssl /usr/share/doc/openssl-3.4.0
```

