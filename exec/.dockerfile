FROM rustlang/rust:nightly as builder
LABEL stage=builder

WORKDIR /usr/src/exec
COPY . .


RUN cargo install --path .

FROM archlinux:base-devel
RUN pacman-db-upgrade
RUN pacman -Syu --noconfirm

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
ENV NODE_VERSION=17.7.1
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -q --no-modify-path -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-play
RUN pacman -S python --noconfirm
COPY Rocket.toml .
COPY --from=builder /usr/local/cargo/bin/exec /usr/local/bin/exec
CMD ["exec"]