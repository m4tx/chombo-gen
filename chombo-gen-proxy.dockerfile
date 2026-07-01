FROM nginx:1.31
COPY proxy/default.conf /etc/nginx/conf.d/default.conf
