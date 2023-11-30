#!/bin/bash

PRESTASHOP_CONTAINER_ID=$1
docker cp classic.zip $PRESTASHOP_CONTAINER_ID:/var/www/html/themes/classic.zip 
docker cp child_classic.zip $PRESTASHOP_CONTAINER_ID:/var/www/html/themes/child_classic.zip 
docker exec $PRESTASHOP_CONTAINER_ID bash -c "unzip /var/www/html/themes/classic.zip -d /var/www/html/themes"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "unzip /var/www/html/themes/child_classic.zip -d /var/www/html/themes"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm /var/www/html/themes/child_classic.zip"
docker exec $PRESTASHOP_CONTAINER_ID bash -c "rm /var/www/html/themes/classic.zip"