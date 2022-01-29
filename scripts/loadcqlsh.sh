#!/bin/bash

docker exec cassandra /bin/bash -c "cqlsh --file=/scripts/data.cql"
