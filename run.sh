#!/bin/bash

#if [[  -x "$(command -v podman)" ]]; then
	#ENGINE="podman"
#else
	ENGINE="docker"
#fi

$ENGINE run --rm -d -v $(pwd)/ctfwatch:/ctfwatch -p 8081:80 --name ctfwatch ctfwatch
