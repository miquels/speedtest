FROM golang

# install build tools
RUN apt-get update
RUN apt-get install -y git build-essential cmake

# copy server-go files
WORKDIR /go/src/server-go
COPY . .

# build server 
RUN make install

EXPOSE 4000
CMD ["server-go"]
