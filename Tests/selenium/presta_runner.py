from selenium.common import NoSuchElementException, StaleElementReferenceException
from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC, expected_conditions
from selenium.webdriver.common.keys import Keys
import time
import random
import string

class PrestaRunner():
    __GO_TO_ORDER_XPATH = '//*[@id="blockcart-modal"]/div/div/div[2]/div/div[2]/div/div/a'
    __SEARCHED_PRODUCT_NAME = 'oprawki'

    def __init__(self, web_driver, url_base):
        self.web_driver = web_driver
        self.url_base = url_base

    def add_10_products_to_cart(self):
        self.__select_first_category()
        self.__select_products_from_category(6)
        self.__select_second_category()
        self.__select_products_from_category(4)

    def search_for_product_and_add_to_cart(self):
        search_field = self.web_driver.find_element(By.XPATH, '//*[@id="search_widget"]/form/input[2]')
        search_field.send_keys(self.__SEARCHED_PRODUCT_NAME)
        search_field.send_keys(Keys.ENTER)
        self.__go_to_rand_element()
        self.__add_to_cart(1)

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
        self.__choose_delivery()
        self.__finalize_delivery()

    def get_order_details(self):
        self.__go_to_order_details()
        self.__download_vat_invoice()

    def __download_vat_invoice(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="order-infos"]/div[2]/ul/li[3]/a').click()

    def __go_to_order_details(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="_desktop_user_info"]/div/a[2]/span').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="history-link"]/span/i').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="content"]/table/tbody/tr/td[6]/a[1]').click()

    def __finalize_delivery(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="payment-option-2"]').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="conditions_to_approve[terms-and-conditions]"]').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="payment-confirmation"]/div[1]/button').click()

    def __choose_delivery(self):
        self.web_driver.find_element(By.XPATH, '//*[@id="delivery_option_11"]').click()
        self.web_driver.find_element(By.XPATH, '//*[@id="js-delivery"]/button').click()

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
        email = 'andrzej.' + self.__get_random_email_elem() + '@wp.pl'
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
        xpath = f'//*[@id="js-product-list"]/div[1]/div[{rand_index}]/article/div/div[2]/h2/a'
        self.web_driver.find_element(By.XPATH, xpath).click()

    def __select_first_category(self):
        actions = ActionChains(self.web_driver)
        cat_elem = self.web_driver.find_element(By.XPATH, '//*[@id="category-69"]/a')
        actions.move_to_element(cat_elem)
        category = self.web_driver.find_element(By.XPATH, '//*[@id="category-69"]/a')
        actions.click(category)
        actions.perform()

    def __select_second_category(self):
        actions = ActionChains(self.web_driver)
        cat_elem = self.web_driver.find_element(By.XPATH, '//*[@id="category-69"]/a')
        actions.move_to_element(cat_elem)
        category = self.web_driver.find_element(By.XPATH, '//*[@id="category-70"]/a')
        actions.click(category)
        actions.perform()

    def __select_products_from_category(self, count):
        i = 0
        more_than_one_added = False
        while i < count:
            prd_count = 1
            if not more_than_one_added:
                prd_count = 2
            try:
                self.__go_to_nth_product_and_add_to_cart(i + 1, prd_count)
                if prd_count > 1:
                    more_than_one_added = True
            except Exception as e:
                count += 1
                self.web_driver.back()
            i += 1

    def __go_to_nth_product_and_add_to_cart(self, index, count):
        xpath = self.__get_nth_element_xpath(index)
        product_elem = self.web_driver.find_element(By.XPATH, xpath)
        product_elem.click()
        self.__add_to_cart(count)

    def __add_to_cart(self, count):

        #increase_count_button = self.web_driver.find_element(By.XPATH,
        #                                                     '//*[@id="add-to-cart-or-refresh"]/div[2]/div[1]/div[1]/div/span[3]/button[1]')
        if count > 1:
            input_elem = self.web_driver.find_element(By.XPATH, '//*[@id="quantity_wanted"]')
            input_elem.send_keys(Keys.CONTROL + "a", Keys.BACKSPACE)
            input_elem.send_keys(str(count) + Keys.ENTER)

        ignored_exceptions = (NoSuchElementException, StaleElementReferenceException)
        add_to_cart_button = WebDriverWait(self.web_driver, 10, ignored_exceptions=ignored_exceptions) \
            .until(expected_conditions.presence_of_element_located((By.CLASS_NAME,
                                                          'btn.btn-primary.add-to-cart')))

        add_to_cart_button.click()
        WebDriverWait(self.web_driver, 5).until(EC.presence_of_element_located((By.XPATH, self.__GO_TO_ORDER_XPATH)))
        self.web_driver.back()

    def __get_nth_element_xpath(self, index):
        return f'//*[@id="js-product-list"]/div[1]/div[{index}]/article/div/div[1]/a/img'

    def __get_random_email_elem(self):
        length = 10
        return ''.join(random.choices(string.ascii_letters, k=length)).lower()
