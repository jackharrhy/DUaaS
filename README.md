# **DUaaS** ➡ _Dev Urandom as a Service_
/dev/urandom as a service

<a href="https://hub.docker.com/r/jackharrhy/duaas"><a href="https://microbadger.com/images/jackharrhy/duaas" title="Get your own image badge on microbadger.com"><img src="https://images.microbadger.com/badges/image/jackharrhy/duaas.svg"></a></a>

![a selection of GET /ecoji/1000 displayed in a browser](https://i.imgur.com/KF4gskb.jpg)

### GET /binary

> read and return 128 lines from /dev/urandom as binary
> formatted as a string

### GET /binary/\<usize>

> read and return \<usize> number of lines from
> /dev/urandom as binary formatted as a string

### GET /base64

> read and return 128 lines from /dev/urandom as base64

### GET /base64/\<usize>

> read and return \<usize> number of lines from
> /dev/urandom as base64

### GET /ecoji

> read and return 128 lines from /dev/urandom as base64,
> interpreted as emoji using ecoji

### GET /ecoji/\<usize>

> read and return \<usize> number of lines from
> /dev/urandom as base64 interpreted as emoji using ecoji"
