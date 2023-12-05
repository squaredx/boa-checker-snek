####################################################################################################
## Builder - aarch64-unknown-linux-gnu
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add aarch64-unknown-linux-gnu
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --target aarch64-unknown-linux-gnu --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/boa-checker-snek ./

# Use an unprivileged user.
USER app:app

EXPOSE 8000

CMD ["/app/boa-checker-snek"]