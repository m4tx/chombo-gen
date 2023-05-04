FROM nginx:1.24
COPY proxy/default.conf /etc/nginx/conf.d/default.conf
