Projekt na biznes elektroniczny

Prestashop zawiera kod sklepu, a także skrypty dotyczące instalacji ssl (w folderze .docker)

Folder ze skryptami zawiera skrypty dotyczące dumpowania bazy.

W celu wykonania dumpa należy:

```
Upewnić się, że skrypt ma poprawnego chmoda
Uruchomić sklep przy użyciu docker-compose up -d
Przy pomocy polecenia docker ps sprawdzić ID kontenera z bazą
Należy wywołać skrypt create_dump.sh podając argumenty: id kontenera, hasło do bazy, nazwę bazy.
```

Jeśli chodzi o id kontenera, to wystarczą pierwsze litery pozwalające odróżnić go od pozostałych, np jeżeli mamy konetenery o id:
2ac4cde,
2abcijk (mariadb),
zcdefjk, to wystarczyłoby podać id 2ab.

W celu załadowania dumpa należy:

```
Upewnić się, że skrypt ma poprawnego chmoda
Uruchomić sklep przy użyciu docker-compose up -d
Przy pomocy polecenia docker ps sprawdzić ID kontenera z bazą
Należy wywołać skrypt load_dump.sh podając argumenty: id kontenera, hasło do bazy.
```
