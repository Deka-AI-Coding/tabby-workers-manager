FROM rust:1.75 AS build

WORKDIR /src
COPY Cargo.lock Cargo.toml ./

RUN cargo fetch --locked

COPY . .

RUN cargo build --release --frozen --offline


FROM rust:1.75 AS runtime

COPY --from=build /src/target/release/tabby_worker_manager_cli /usr/local/bin/

ENV PATH=$PATH:/usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tabby_worker_manager_cli" ]