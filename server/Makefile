#! /bin/sh

export GOPATH = $(HOME)

all:	get build

install:	all

build:
	@echo "server: building"
	go build

get:
	@echo "server: getting dependencies.."
	go get

.PHONY:	get build install
