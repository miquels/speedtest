# speedtest

> HTML5 Speedtest

This speedtest uses [the same algorithm](doc/algorithm.md) as the Ookla / speedtest.net test. Because of that it gives about the same results.

It has a responsive interface, and can be used on desktop, tablets and phones.

![speedtest-dark](https://user-images.githubusercontent.com/6455542/83277443-589b5380-a1d2-11ea-83e6-c620326ed3e1.png)

## Development / Build Setup

- as usual, you need nodejs and yarn
- you need to have Rust (compiler + cargo) installed, version 1.40 or later.
  Debian 11 (bullseye) (to be released in 2021) has a modern enough Rust.
  Otherwise, go to https://rustup.rs/ and follow the (simple) install instructions.

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

# build for production with minification
yarn build
```

## Run the server
``` bash
# build server
cd server
source ~/.cargo/env # optional
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
# build
yarn build

# copy files to your webservers root. change /path/to/www/html to the
# actual path on your system (e.g. debian: /var/www/html).
cp -av dist/* /path/to/www/html/
cp -a public/{index.html,config.json} /path/to/www/html/
```

# Running the API server.

The compiled server binary can be found in this location:
server/target/release/speedtest-server. Copy it to a generic location,
like /usr/local/sbin. It needs to be run as a daemon. That's OS specific,
and no sysv / systemd / whatever files have been included yet. The easiest
solution is to run it in a ``screen'' session for now :)

The server has TLS (SSL) support -- run it with '--help' for more info.
You can also use it to serve the entire application, using the --dir option.
You don't get any logging in that case though, so if you need that,
serve the application using a generic webserver like Apache or Nginx,
and run the api server on a separate port.

