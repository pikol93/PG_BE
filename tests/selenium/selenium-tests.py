from selenium import webdriver

driver_path = '/usr/bin/chromedriver'
driver = webdriver.Chrome(driver_path)

driver.get('https://pg.edu.pl/')
