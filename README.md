# **DUaaS** ➡ _Dev Urandom as a Service_
/dev/urandom as a service

[![Build Status](https://drone.jackharrhy.com/api/badges/jackharrhy/DUaaS/status.svg)](https://drone.jackharrhy.com/jackharrhy/DUaaS)

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
