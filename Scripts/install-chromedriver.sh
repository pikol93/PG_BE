#!/bin/bash

# check if sudo
if [ "$EUID" -ne 0 ] 
	then echo "Please run as root"
    exit
fi

# check arguments count
if [ "$#" -lt 1 ]
    then echo "Provide full chromedriver version as script argument"
    exit
fi

# version can be checked here: https://googlechromelabs.github.io/chrome-for-testing/#stable 

# install chrome webdriver
CHROMEDRIVER_VERSION="$1"

# download
cd /tmp
wget -N "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/$CHROMEDRIVER_VERSION/linux64/chromedriver-linux64.zip"
unzip chromedriver-linux64.zip
sudo mv -f chromedriver-linux64/chromedriver /usr/local/share/
sudo chmod +x /usr/local/share/chromedriver

