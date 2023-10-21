Instalacja
--------

Podczas pierwszego uruchomienia, należy zmienić zmienną środowiskową DISABLE_MAKE w serwisie prestashop-git:

      DISABLE_MAKE: ${DISABLE_MAKE:-0}
Po pierwszym uruchomieniu, zmienną tą należy ponownie zmodyfikować:

      DISABLE_MAKE: ${DISABLE_MAKE:-1}


Uruchomienie
--------

Aby uruchomić projekt, należy skorzystać z komendy:

    docker-compose up
Do zatrzymania projektu, należy skorzystać z komendy:

    docker-compose down

Komendę docker-compoese down trzeba wywołac z innego terminala.

Przydatne informacje o kontenerach i obrazach
--------

Podczas korzystania z dockera od razu spotka się z pojęciem obrazu i konteneru. Można to trochę porównać do pojęcia klasy i jej instancji. Obraz to przepis na tworzenie kontenerów. Kontenery są od siebie niezależne, tzn. można uruchomić kilka instancji obrazu na raz. Obrazy buduje się z pliku Dockerfile. Polecam komendę --help, która można dodać do samego docker, lub do kazdej z poniższych komend.

Jeśli chodzi o obrazy, to przydatne mogą być komendy:

    docker rmi
    docker images
    docker build
  
Jeśli chodzi o kontenery, to przydać się moga:

    docker run
    docker ps
    docker rm

Może się zdarzyć tak, że dwa kontenery są od siebie zależne np. backend od bazy danych. Żeby nie wpisywać wszystkiego ręcznie i nie musieć dbać o kolejność, korzysta się z narzędzia docker-compose, w którym deklaruje się serwisy. Depends-on pozwala na zachowanie kolejności uruchomień. Serwisy w docker-compose powiązane są ze sobą siecią (network), dzięki czemu mogą się ze sobą komunikować i nie muszą wystawiać endpointów na zewnątrz. Bez docker-compose też da się je tak powiązać, natomiast sieć taką trzeba wtedy stworzyć samemu.

UWAGA - u nas raczej nie trzeba będzie używać dockera, wystarczy docker compose, przynajmniej jeśli 1 etap.

Przydatne informacje o wolumenach i portach
--------

W dockerze wyróżniamy 3 rodzaje wolumenów: anonymous volume, named volume, bind mount.
Pierwszy z nich powstaje, gdy nie nazwiemy wolumenu. Docker compose za każdym razem będzie tworzył nowy wolumen anonimowy, zamiast korzystać z poprzedniego, dlatego odradzam ich używania w tym projekcie.
Drugi z nich, czyli nazwany, powstaje gdy nadamy nazwę wolumenowi. Będzie używany między każdym uruchomieniem, natomiast ciężko będzie znaleźć pliki z wolumenu na dysku. Będziemy zapewne używać go w 2 etapie, aby nie zgubić danych z bazy na clustrze przy wysyłaniu jej tam z dev.
Trzecią opcją, często wykorzystywaną w środowisku deweloperskim, jest wolumen związany z konkretną ścieżką na dysku. Podawana jest ścieżka relatywna do folderu i folder ten służy nam jak wolumen. Ułatwia to tworzenie kopii aczkolwiek grozi nieumyślnym skasowaniem danych.

Przykłady wolumenów w pliku docker-composer:

    anonymouse: /workdir/

    named: imie:/workdir/

    bind-mount: ./ścieżka:/workdir/

Uwaga! W przypadku używania wolumenów nazwanych, trzeba je zadeklarować w sekcji pliku volumes:
Jeśli chodzi o bind-mounty, to w przypadku docker run najlepiej podawać ścieżkę absolutną.

