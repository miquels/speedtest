# speedtest

> HTML5 Speedtest

This speedtest uses the same algorithm as the Ookla / speedtest.net test. Because of that it gives about the same results.

It has a responsive interface, and can be used on desktop, tablets and phones.

![screenshot](https://cloud.githubusercontent.com/assets/6455542/22569035/6288b92a-e996-11e6-92a6-20ff57676e13.png)

## Development / Build Setup

``` bash
# install dependencies
yarn

# install configuration file
cp static/config.json.example static/config.json
vim static/config.json

# serve with hot reload at localhost:8080
yarn run dev

# build for production with minification
yarn run build
```

## Run the server
``` bash
# build server
yarn run build-server

# run server
server/server
```

