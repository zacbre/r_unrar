# r_unrar
unRAR completed torrent files with ease.

to compile for x86_64 linux gnu:
```
cargo build --release --target=x86_64-unknown-linux-gnu
# optional strip for smaller binary
x86_64-unknown-linux-gnu-strip -s target/x86_64-unknown-linux-gnu/release/r_unrar
```

default will compile for musl, so musl libs/linker/tools needed (musl-gcc, etc).
```
# optional musl strip
x86_64-linux-musl-strip -s target/x86_64-unknown-linux-musl/release/r_unrar
```

add to completed downloads execute via qbittorrent with:
```
r_unrar '%R'
```

or add to rtorrent with completed downloads handler:
```
method.set_key = event.download.finished,r_unrar,"execute=/path/to/r_unrar,$d.base_path="
```