####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS base

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=terra
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN cargo install cargo-chef
WORKDIR /exec

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /exec/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release --bin execution

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /exec

# Copy our build
COPY --from=builder /exec/target/x86_64-unknown-linux-musl/release/execution ./

# Use an unprivileged user.
USER terra:terra

CMD ["/exec/execution"]
