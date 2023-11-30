#!/bin/bash

PRESTASHOP_CONTAINER_ID=$1
cat images.tar.gz.part* > images.tar.gz
docker cp ./images.tar.gz $PRESTASHOP_CONTAINER_ID:/tmp/images.tar.gz
rm images.tar.gz
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm -rf /var/www/html/img/*"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "tar -xzvf /tmp/images.tar.gz -C /"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm -rf /tmp/images.tar.gz"