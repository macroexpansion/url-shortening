#!/bin/bash

docker run --rm -it --network url-shortening_local nuvo/docker-cqlsh cqlsh cassandra 9042 --cqlversion='3.4.5'
