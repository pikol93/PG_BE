# To run files with the same group as your primary user
ARG VERSION

FROM prestashop/prestashop-git:$VERSION

ARG GROUP_ID
ARG USER_ID

# To run files with the same group as your primary user
RUN groupmod -g $GROUP_ID www-data \
  && usermod -u $USER_ID -g $GROUP_ID www-data

RUN mkdir -p /var/www/.npm
RUN chown -R www-data:www-data /var/www/.npm

# These two directories are docker mounted volumes
RUN chown -R www-data:www-data /var/www/html/vendor
RUN chown -R www-data:www-data /var/www/html/var

RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y nodejs

# Install mailutils to make sendmail work
RUN apt install -y \
    apt-utils \
    mailutils

RUN apt update && apt install -y --no-install-recommends \
        libc-client-dev \
        libkrb5-dev \
        ; \
        rm -rf /var/lib/apt/lists/*

RUN set -eux; \
	PHP_OPENSSL=yes docker-php-ext-configure imap --with-kerberos --with-imap-ssl; \
	docker-php-ext-install imap

COPY .docker/wait-for-it.sh /tmp/
COPY .docker/docker_run_git.sh /tmp/
COPY .docker/ssl.sh /tmp/
COPY .docker/000-default.conf /tmp/

CMD ["/tmp/docker_run_git.sh"]
