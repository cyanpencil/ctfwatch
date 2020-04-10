#!/bin/bash

cd ctfwatch

touch out
( while true; do cargo run > out; sleep 300; done ) &

sleep 5

socat tcp-l:80,reuseaddr,fork system:"echo HTTP/1.0 200; echo Content-Type\: text/plain; echo; cat out"
