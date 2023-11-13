#!/bin/bash

CONTAINER_NAME=$1
PASS=$2
DATABASE=$3
docker exec $CONTAINER_NAME bash -c "usr/bin/mysqldump -u root --password=${PASS} --databases ${DATABASE}" > ./dump.sql