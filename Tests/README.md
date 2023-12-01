# Instrukcja jak pobrać webdriver na linuxie



## Sprawdzenie wersji chrome

Wpisujemy w terminal

> google-chrome --version

dostaniemy coś w stylu 

> 119.x.x.x

Generalnie tylko pierwsza człon jest ważny.

Wchodzimy na https://googlechromelabs.github.io/chrome-for-testing/#stable i szukamy tam, jaka jest dokładnie dostępna wersja z naszym prefiksem, przykładowo dla 119 jest to

> 119.0.6045.105

## Instalacja chromedriver 

Wchodzimy do folderu Scripts i uruchamiamy jako root skrypt **install-chromedriver.sh** podając jako argument odpowiednią wersję chromedrivera, przykładowo

> sudo ./install-chromedriver.sh 119.0.6045.105

Odpowiedni sterownik powinien sie po chwili zainstalować.

## Instalacja Selenium

Wystarczy uruchomić 

> pip install -U selenium



## Uruchomienie testów

Aby uruchomić testy wystarczy przejść do folderu **Tests/selenium** i wykonać

> python3 selenium-tests.py