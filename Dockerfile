FROM node:16-alpine as frontend
WORKDIR /frontend
COPY /frontend .
RUN npm i
RUN npm run generate

FROM rust:1.67
WORKDIR /usr/src/webprogrammierung_group_5
COPY . .
COPY --from=frontend /frontend/dist dist
EXPOSE 8000
RUN rustup default nightly

CMD cargo run
