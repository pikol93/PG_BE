from selenium import webdriver

class ChromeDriverProvider():
    def build_chromedriver(self):
        driver_path = '/usr/bin/chromedriver'
        options = webdriver.ChromeOptions()
        options.add_argument('--ignore-ssl-errors=yes')
        options.add_argument('--ignore-certificate-errors')
        return webdriver.Chrome(driver_path, options=options)
