FROM rust:alpine as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache libgcc

COPY --from=build /app/target/release/shape_area_calculator /app/shape_area_calculator

CMD [ "/app/shape_area_calculator" ]