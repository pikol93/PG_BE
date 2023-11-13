#!/bin/bash

CONTAINER_NAME=$1
PASS=$2
docker cp ./dump.sql $CONTAINER_NAME:/tmp/dump.sql
docker exec $CONTAINER_NAME bash -c "mysql -u root --password=${PASS}  < /tmp/dump.sql"