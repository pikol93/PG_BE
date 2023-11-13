#!/bin/bash

mkdir /tmp/ssl
touch /tmp/ssl/apache-selfsigned.key
touch /tmp/ssl/apache-selfsigned.crt
openssl req -newkey rsa:2048 -new -nodes -x509 -days 365 -keyout /tmp/ssl/apache-selfsigned.key -out /tmp/ssl/apache-selfsigned.crt -subj "/C=PL/ST=Pomerania/L=Gdansk/O=PG/OU=./CN=."
cp /tmp/ssl/apache-selfsigned.key /etc/ssl/private/apache-selfsigned.key
cp /tmp/ssl/apache-selfsigned.crt /etc/ssl/certs/apache-selfsigned.crt
