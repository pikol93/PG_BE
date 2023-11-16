from chromedriver_provider import ChromeDriverProvider
from presta_runner import PrestaRunner
import time

driver = ChromeDriverProvider().build_chromedriver()
url_base = 'https://localhost:8001/'
presta_runner = PrestaRunner(driver, url_base)

driver.get(url_base)

presta_runner.add_10_products_to_cart()
presta_runner.search_for_product_and_add_to_cart()
presta_runner.delete_3_products_from_cart()
presta_runner.register_new_account()
time.sleep(15)

driver.close()
