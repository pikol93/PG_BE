#!/bin/bash

DATABASE_CONTAINER_ID=$1
PRESTASHOP_CONTAINER_ID=$2
PASS=$3
./database_update.sh $DATABASE_CONTAINER_ID $PASS
./set_images.sh $PRESTASHOP_CONTAINER_ID
./update_themes.sh $PRESTASHOP_CONTAINER_ID