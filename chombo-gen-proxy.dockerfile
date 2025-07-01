FROM nginx:1.29
COPY proxy/default.conf /etc/nginx/conf.d/default.conf
