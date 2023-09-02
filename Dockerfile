# builder
FROM rust:1.55.0-alpine3.14 as builder
LABEL authors="Milan Jaric"

COPY . /usr/src/schedule-rs

WORKDIR /usr/src/schedule-rs

RUN cargo build --release


# runtime
FROM alpine:3.14
ARG VERSION=0.1.0

LABEL authors="Milan Jaric"
LABEL description="ScheduleRS is a simple tool for scheduling commands to run at a given time."
LABEL vcs-url="https://github.com/0xPANSE/schedule-rs"
LABEL version=${VERSION}

# override to change the default path
ENV SHEDULE_RS_DB_PATH=/var/lib/schedule-rs/data

RUN apk add --no-cache ca-certificates
RUN mkdir -p /var/lib/schedule-rs/data

COPY --from=builder /usr/src/schedule-rs/target/release/schedule-rs /usr/local/bin/schedule-rs

ENTRYPOINT ["/usr/local/bin/schedule-rs"]