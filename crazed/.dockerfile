FROM rustlang/rust:nightly as builder
LABEL stage=builder

WORKDIR /usr/src/crazed
COPY . .

RUN cargo install --path .

FROM archlinux:base-devel
RUN pacman-db-upgrade
RUN pacman -Syu --noconfirm
COPY --from=builder /usr/local/cargo/bin/crazed /usr/local/bin/crazed
COPY --from=builder /usr/src/crazed/.env ./.env
CMD ["crazed"]