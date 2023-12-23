#!/bin/bash

PASS=$1
#mysql -h localhost -P 3306 --protocol=tcp -u root --password=${PASS}  < ./dump/dump.sql

DATABASE_CONTAINER_ID=$2
docker cp ./dump/dump.sql $DATABASE_CONTAINER_ID:/tmp/dump.sql
docker exec $DATABASE_CONTAINER_ID bash -c "mysql -u root --password=${PASS}  < /tmp/dump.sql"
docker exec $DATABASE_CONTAINER_ID bash -c "rm -rf /tmp/dump.sql"