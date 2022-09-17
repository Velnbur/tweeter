FROM ghcr.io/velnbur/trunk:master as builder

ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models

ADD ./apps/tweeter-frontend /usr/src/apps/tweeter-frontend

WORKDIR /usr/src/apps/tweeter-frontend

RUN trunk build --release

# -----
FROM nginx:stable-alpine

COPY --from=builder /usr/src/apps/tweeter-frontend/dist /bin/www

COPY ./apps/tweeter-frontend/configs/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD [ "nginx", "-g", "daemon off;" ]
