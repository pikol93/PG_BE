Projekt na biznes elektroniczny
------

Skład zespołu
---
Zespół tworzony jest przez studentów:
- Sławomir Adamowicz
- Piotr Dampc
- Karol Felskowski
- Jakub Deniziak
- Michał Szulowski

Struktura projektu
---

Prestashop zawiera kod sklepu, a także skrypty dotyczące instalacji ssl (w folderze .docker)

Folder Scripts zawiera skrypty dotyczące dumpowania bazy.


Dump
---
W celu wykonania dumpa należy:

```
Upewnić się, że skrypt ma poprawnego chmoda
Uruchomić sklep przy użyciu docker-compose up -d
Przy pomocy polecenia docker ps sprawdzić ID kontenera z bazą i z prestą
Należy wywołać skrypt create_dump.sh podając argumenty: id kontenera z bazą, id kontenera z prestą, hasło do bazy, nazwę bazy.
```

Jeśli chodzi o id kontenera, to wystarczą pierwsze litery pozwalające odróżnić go od pozostałych, np jeżeli mamy konetenery o id:
2ac4cde,
2abcijk (mariadb),
zcdefjk, to wystarczyłoby podać id 2ab. (bo to hashe)

W celu załadowania dumpa należy:

```
Upewnić się, że skrypt ma poprawnego chmoda
Uruchomić sklep przy użyciu docker-compose up -d
Przy pomocy polecenia docker ps sprawdzić ID kontenera z bazą i z prestą
Należy wywołać skrypt load_dump.sh podając argumenty: id kontenera z bazą, id kontenera z prestą, hasło do bazy.
```
