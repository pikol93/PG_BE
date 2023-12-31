#!/bin/bash

(sleep 10 && echo jazda && ./baza.sh student baza_188581) &
docker stack deploy -c docker-compose.yaml BE_18858 --with-registry-auth
