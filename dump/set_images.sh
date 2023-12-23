#!/bin/bash

cat /tmp/dump/images.tar.gz.part* > /tmp/images.tar.gz
rm -rf /var/www/html/img/*
tar -xzvf /tmp/images.tar.gz -C /