####################################################################################################
## Builder
####################################################################################################
FROM rustlang/rust:nightly AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=rustuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /api

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release
#RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
#FROM debian:buster-slim
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /api

# Copy our build
COPY --from=builder /api/target/x86_64-unknown-linux-musl/release/api ./

# Use an unprivileged user.
USER rustuser:rustuser

CMD ["/api/api"]
