from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.keys import Keys
import time
import random

class PrestaRunner():
    __GO_TO_ORDER_XPATH = '//*[@id="blockcart-modal"]/div/div/div[2]/div/div[2]/div/div/a'
    __SEARCHED_PRODUCT_NAME = 'mug'

    def __init__(self, web_driver, url_base):
        self.web_driver = web_driver
        self.url_base = url_base

    def add_10_products_to_cart(self):
        self.__select_first_category()
        self.__select_products_from_category(2)
        self.__select_second_category()
        self.__select_products_from_category(2)

    def search_for_product_and_add_to_cart(self):
        search_field = self.web_driver.find_element(By.XPATH, '//*[@id="search_widget"]/form/input[2]')
        search_field.send_keys(self.__SEARCHED_PRODUCT_NAME)
        search_field.send_keys(Keys.ENTER)
        self.__go_to_rand_element()
        self.__add_to_cart()

    def delete_3_products_from_cart(self):
        self.__go_to_cart()
        delete_button_xpath = '//*[@id="main"]/div/div[1]/div/div[2]/ul/li[1]/div/div[3]/div/div[3]/div/a/i'
        for i in range(3):
            delete_button = self.web_driver.find_element(By.XPATH, delete_button_xpath)
            delete_button.click()
            # TODO WARNING IT NEEDS REFACTORING, THIS IS UNSAFE AND MIGHT NOT WORK
            time.sleep(1)

    def register_new_account(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="_desktop_user_info"]/div/a/span').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="content"]/div/a').click()
        self.__fill_personal_data()
        time.sleep(10)

    def make_order(self):
        self.__go_to_cart()
        self.web_driver.find_element(By.XPATH, '//*[@id="main"]/div/div[2]/div[1]/div[2]/div/a').click()
        self.__fill_address_data()
        self.web_driver.find_element(By.XPATH, '//*[@id="main"]/div/div[2]/div[1]/div[2]/div/a').click('//*[@id="delivery_option_11"]')


    def __fill_address_data(self):
        address = 'ul. Warszawska 1'
        zip_code = '01-001'
        city = 'Warszawa'

        self.web_driver.find_element(By.XPATH, '//*[@id="field-address1"]').send_keys(address)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-postcode"]').send_keys(zip_code)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-city"]').send_keys(city)

        self.web_driver.find_element(By.XPATH, '//*[@id="delivery-address"]/div/footer/button').click()


    def __fill_personal_data(self):
        name = 'Andrzej'
        second_name = 'Warszawski'
        email = 'andrzej.warszawski@wp.pl'
        password = 'passwordi3402'
        birth_date = '2000-01-01'

        self.web_driver.find_element(By.XPATH, '//*[@id="field-id_gender-1"]').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="field-firstname"]').send_keys(name)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-lastname"]').send_keys(second_name)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-email"]').send_keys(email)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-password"]').send_keys(password)
        self.web_driver.find_element(By.XPATH, '//*[@id="field-birthday"]').send_keys(birth_date)

        self.web_driver.find_element(By.XPATH, '//*[@id="customer-form"]/div/div[8]/div[1]/span/label/input').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="customer-form"]/div/div[10]/div[1]/span/label/input').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="customer-form"]/footer/button').click()

    def __go_to_cart(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="_desktop_cart"]/div/div/a/i').click()

    def __go_to_rand_element(self):
        elements_div = self.web_driver.find_element(By.XPATH, '//*[@id="js-product-list"]/div[1]')
        products_count = len(elements_div.find_elements(By.XPATH, "./*"))
        starting_index = 1
        rand_index = starting_index + random.randint(0, products_count - 1)
        xpath = f'//*[@id="js-product-list"]/div[1]/div[{rand_index}]/article/div/div[1]/a/img'
        self.web_driver.find_element(By.XPATH, xpath).click()

    def __select_first_category(self):
        actions = ActionChains(self.web_driver)
        cat_elem = self.web_driver.find_element(By.XPATH, '//*[@id="category-3"]/a')
        actions.move_to_element(cat_elem)
        category = self.web_driver.find_element(By.XPATH, '//*[@id="category-4"]/a')
        actions.click(category)
        actions.perform()

    def __select_second_category(self):
        actions = ActionChains(self.web_driver)
        cat_elem = self.web_driver.find_element(By.XPATH, '//*[@id="category-3"]/a')
        actions.move_to_element(cat_elem)
        category = self.web_driver.find_element(By.XPATH, '//*[@id="category-5"]/a')
        actions.click(category)
        actions.perform()

    def __select_products_from_category(self, count):
        for i in range(count):
            self.__go_to_nth_product_and_add_to_cart(i + 1)

    def __go_to_nth_product_and_add_to_cart(self, index):
        xpath = self.__get_nth_element_xpath(index)
        product_elem = self.web_driver.find_element(By.XPATH, xpath)
        product_elem.click()
        self.__add_to_cart()

    def __add_to_cart(self):
        add_to_cart_button = self.web_driver.find_element(By.XPATH,
                                                          '//*[@id="add-to-cart-or-refresh"]/div[2]/div/div[2]/button')
        add_to_cart_button.click()
        WebDriverWait(self.web_driver, 5).until(EC.presence_of_element_located((By.XPATH, self.__GO_TO_ORDER_XPATH)))
        self.web_driver.back()

    def __get_nth_element_xpath(self, index):
        return f'//*[@id="js-product-list"]/div[1]/div[{index}]/article/div/div[1]/a/img'
