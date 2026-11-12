#!/bin/sh


export NGINX_WEB_ROOT=${NGINX_WEB_ROOT:-'/var/www/html'}
export NGINX_USER=${NGINX_USER:-'www-data'}
export NGINX_CONF=${NGINX_CONF:-'/etc/nginx/nginx.conf'}

sed -i "s/http:\\/\\/127\.0\.0\.1/${URL_ESC}/" /usr/share/nginx/html/doc/*
sed -i "s/http:\\/\\/127\.0\.0\.1/${URL_ESC}/" /usr/share/nginx/html/openapi/openapi.json
sed -i "s/http:\\/\\/127\.0\.0\.1/${URL_ESC}/" /usr/share/nginx/html/openapi/openapi.yaml

/usr/bin/supervisord -c /etc/supervisor/conf.d/supervisor.conf
