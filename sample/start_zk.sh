#!/bin/sh
docker run -it -p 2181:2181 --rm -v $(dirname $0)/data:/data -v $(dirname $0)/datalog:/datalog zookeeper
