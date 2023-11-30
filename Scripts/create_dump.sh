#!/bin/bash

DATABASE_CONTAINER_ID=$1
PRESTASHOP_CONTAINER_ID=$2
PASS=$3
DATABASE=$4
docker exec $PRESTASHOP_CONTAINER_ID bash -c "tar -czvf images.tar.gz /var/www/html/img/"
docker cp $PRESTASHOP_CONTAINER_ID:/var/www/html/images.tar.gz images.tar.gz
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm -rf images.tar.gz"
rm images.tar.gz.part*
split -b 20M images.tar.gz "images.tar.gz.part"
rm images.tar.gz
docker exec $DATABASE_CONTAINER_ID bash -c "usr/bin/mysqldump -u root --password=${PASS} --databases ${DATABASE}" > ./dump.sql