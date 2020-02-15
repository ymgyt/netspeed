# netspeed

## Install

### `cargo`

```console
# install
$ cargo install netspeed

# update
$ cargo install --force netspeed
```

### MacOS

```console
$ brew tap ymgyt/netspeed
$ brew install netspeed
```


## Usage

```console
$ netspeed                                                                                                                                                     +[master]
INFO  Connecting to "netspeed.ymgyt.io:5555"
INFO  Start downstream duration: 3 seconds
INFO  Start upstream duration: 3 seconds
Downstream: 24.00 Mbps
  Upstream: 106.67 Mbps
```


### running server

terminal1
```console
$ netspeed server run
INFO  2020-02-15T10:13:10.360482+00:00 Listening on "0.0.0.0:5555" max threads: 100
```

terminal2
```console
$ netspeed --addr=127.0.0.1:5555                                                                                                                            +[master]
INFO  Connecting to "127.0.0.1:5555"
INFO  Start downstream duration: 3 seconds
INFO  Start upstream duration: 3 seconds
Downstream: 13.97 Gbps
  Upstream: 17.99 Gbps
```

