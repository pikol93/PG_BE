from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains

class PrestaRunner():
    def __init__(self, web_driver, url_base):
        self.web_driver = web_driver
        self.url_base = url_base

    def add_10_products_to_cart(self):
        self.__select_first_category()

    def search_for_product_and_add_to_cart(self):
        pass

    def delete_3_products_from_cart(self):
        pass

    def register_new_account(self):
        pass

    def make_order(self):
        pass

    def __select_first_category(self):
        actions = ActionChains(self.web_driver)
        cat_elem = self.web_driver.find_element(By.XPATH, '//*[@id="category-3"]/a')
        actions.move_to_element(cat_elem)
        category = self.web_driver.find_element(By.XPATH, '//*[@id="category-4"]/a')
        actions.click(category)
        actions.perform()

    def __select_products_from_category(self, count):
        pass

    def __get_nth_element_xpath(self, index):
        return f'//*[@id="js-product-list"]/div[1]/div[{index}]/article/div/div[1]/a/img'
