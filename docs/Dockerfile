# FROM clux/muslrust:stable AS build
# WORKDIR /app
# RUN cargo install mdbook
# COPY ./ ./
# RUN mdbook build

# FROM lipanski/docker-static-website:2.4.0 AS release
# COPY --from=build /app/book .

FROM lipanski/docker-static-website:2.4.0 AS release
COPY ./book .
