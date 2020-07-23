# speedtest

> HTML5 Speedtest

This speedtest uses [the same algorithm](doc/algorithm.md) as the Ookla / speedtest.net test. Because of that it gives about the same results.

It has a responsive interface, and can be used on desktop, tablets and phones.

![speedtest-dark](https://user-images.githubusercontent.com/6455542/83277443-589b5380-a1d2-11ea-83e6-c620326ed3e1.png)

## Development / Build Setup

- as usual, you need `nodejs` and `yarn`.
- you need to have Rust (compiler + cargo) installed, version 1.40 or later.

Right now no Linux distributions come with a recent enough Rust.
You need to install it manually. Instructions can be found on
https://rustup.rs/, but for Linux and macos it boils down to:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone the repo and build:

``` bash
# clone repo
git clone https://github.com/miquels/speedtest.git
cd speedtest

# install dependencies
yarn

# install configuration file
cp public/config.json.example public/config.json
vim public/config.json

# serve with hot reload at localhost:8080
yarn serve
```

## Run the backend server
``` bash
# build backend server
cd server
source ~/.cargo/env # should not be needed, but try if the below fails.
cargo build --release
cd ..

# run server
server/target/release/speedtest-server
```

If you point yout browser at localhost:8080, you should see the webinterface.

If it doesn't work, use your browsers debugger/inspector, and look at the
javascript console - that should give you a hint as to what is going on.
For example, Chrome on OSX, press Option + Command + J.

## Production use

```
# build and minify.
yarn build

# copy files to your webservers root. change /path/to/www/html to the
# actual path on your system (e.g. debian: /var/www/html).
cp -av dist/* /path/to/www/html/
```

Before you can use the app, you need to run the backend server as well.

# Running the backend server.

If you have compiled the server as described above, its binary needs
to be copied to a generic location, for example:

```
cp server/target/release/speedtest-server /usr/local/sbin/speedtest-server
```

The server needs to be run as a daemon. That's OS specific, and
no sysv / systemd / whatever files have been included yet. The easiest
solution is to run it in a `screen` session for now :)

```
screen
/usr/local/sbin/speedtest-server
# Press "Control-a d" to detach
# Use "screen -x" to re-attach
```

There are several command line options you can use, such as:

- `--key`, `--chain`: options for `TLS` (SSL) support
- `--dir`: serve the entire app, not just the speedtest backend
- `--listen`: address/port to listen on (default 4000)
- `--help`: get a list of all options.

For example, to serve TLS on port 443:

```
/usr/local/sbin/speedtest-server --key certificate.key \
        --chain certificate.pem --listen 443 --dir /var/www/html/speedtest
# Note: set `apiport` in `config.json` to 443, or just comment it out.
```

Note that the server does not write any access logs. If you do need
access logs, you can get them (at least for the application itself,
html/css/javascripts/assets) by serving the application using a
generic webserver like Apache or Nginx, and running the api server
on a separate port.

