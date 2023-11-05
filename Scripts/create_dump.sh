#!/bin/bash

CONTAINER_NAME=$1
PASS=$2
DATABASE=$3
echo "docker exec $CONTAINER_NAME /bin/bash -c usr/bin/mysqldump -u root --password=${PASS} --databases ${DATABASE} > ./dump.sql"
docker exec $CONTAINER_NAME /bin/bash -c "usr/bin/mysqldump -u root --password=${PASS} --databases ${DATABASE}" > ./dump.sql