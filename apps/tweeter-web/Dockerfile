FROM ghcr.io/velnbur/trunk:master as builder

# Copy local dependecies
ADD ./crates/tweeter-schemas /usr/src/crates/tweeter-schemas
ADD ./crates/tweeter-models /usr/src/crates/tweeter-models

# Copy project code
ADD ./apps/tweeter-web /usr/src/apps/tweeter-web

WORKDIR /usr/src/apps/tweeter-web

RUN trunk build --release

# -----
FROM nginx:stable-alpine

COPY --from=builder /usr/src/apps/tweeter-web/dist /bin/www

COPY ./apps/tweeter-web/configs/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD [ "nginx", "-g", "daemon off;" ]
