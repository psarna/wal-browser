# WAL browser

A simple tool for browsing and validating contents of libSQL/SQLite WAL files.

Installation: `cargo install wal-browser`

Usage: `wal-browser /path/to/your/wal/file`

Examples:
```
$ wal-browser /tmp/test-wal 
magic=0x377f0682 version=3007000 page_size=4096 seq=0 salt=d249046c-1c23a039 checksum=40eb4ce5-327347a6
0: page=1 size_after=0
1: page=2 size_after=2
2: page=2 size_after=2
3: page=1 size_after=0
4: page=2 size_after=0
5: page=3 size_after=0
6: page=4 size_after=4
7: page=2 size_after=4
```

```
$ wal-browser /tmp/test2-wal 
magic=0x377f0682 version=3007000 page_size=4096 seq=0 salt=d249046c-1c23a039 checksum=40eb4ce5-327347a6
0: page=1 size_after=0
WARNING: checksum mismatch - file corrupted? 5846130a-9f11e9bb != ec8a510a-24bef2bb
1: page=2 size_after=2
WARNING: checksum mismatch - file corrupted? 7d508ce4-20c1d4e6 != 97b7aae4-531241e6
2: page=2 size_after=2
WARNING: checksum mismatch - file corrupted? 8a2ab9be-9f448297 != ef1b97be-c1cea397
3: page=1 size_after=0
WARNING: checksum mismatch - file corrupted? 164e2e77-f8b54378 != 926bac77-60e8a878
4: page=2 size_after=0
WARNING: checksum mismatch - file corrupted? 965704f0-0cc180a6 != 4d7e02f0-f3f7f9a6
5: page=3 size_after=0
WARNING: checksum mismatch - file corrupted? 64c93a40-e7d6765e != c3ae9840-7a69135e
6: page=4 size_after=4
WARNING: checksum mismatch - file corrupted? 39857390-bdb2a6f6 != 58d41190-43e2b7f6
7: page=2 size_after=4
WARNING: checksum mismatch - file corrupted? 93c66769-032149b0 != 339d2569-8be55eb0
```

```
$ wal-browser /tmp/test
WARNING: invalid magic number - not a WAL file or file corrupted?
WARNING: checksum mismatch - file corrupted? 00000001-00000001 != 60caef03-275bd58b
WARNING: invalid file format version - not a WAL file or file corrupted?
magic=0x53514c69 version=1952784486 page_size=1869770081 seq=1948267264 salt=10000202-00402020 checksum=00000001-00000001
```
