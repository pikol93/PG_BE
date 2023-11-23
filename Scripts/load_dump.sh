#!/bin/bash

DATABASE_CONTAINER_ID=$1
PRESTASHOP_CONTAINER_ID=$2
PASS=$3
docker cp ./dump.sql $DATABASE_CONTAINER_ID:/tmp/dump.sql
docker exec $DATABASE_CONTAINER_ID bash -c "mysql -u root --password=${PASS}  < /tmp/dump.sql"
docker exec $DATABASE_CONTAINER_ID bash -c "rm -rf /tmp/dump.sql"
docker cp ./images.tar.gz $PRESTASHOP_CONTAINER_ID:/tmp/images.tar.gz
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm -rf /var/www/html/img/*"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "tar -xzvf /tmp/images.tar.gz -C /"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm -rf /tmp/images.tar.gz"