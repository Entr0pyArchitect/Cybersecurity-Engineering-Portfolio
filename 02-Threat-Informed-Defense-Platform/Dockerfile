FROM golang:alpine AS build
WORKDIR /app
COPY go.mod ./
RUN go mod download
COPY . .
RUN go build -o /bin/platform ./cmd/platform

FROM alpine:latest
RUN adduser -D appuser
USER appuser
WORKDIR /app
COPY --from=build /bin/platform /platform
EXPOSE 8080
ENV APP_PORT=8080
ENTRYPOINT ["/platform"]
