FROM nginx:1.27
COPY proxy/default.conf /etc/nginx/conf.d/default.conf
