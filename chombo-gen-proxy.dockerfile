FROM nginx:1.25
COPY proxy/default.conf /etc/nginx/conf.d/default.conf
