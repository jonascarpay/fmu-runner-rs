FROM rust:1.79-bullseye

RUN apt-get update && apt-get install -y zsh git llvm-dev libclang-dev clang build-essential ca-certificates curl gdb ssh sudo vim unzip less

################################################################################
# Login Shells for Debugging & Development
################################################################################

RUN echo "export PATH=${CARGO_HOME}/bin:\${PATH}" >> /etc/profile
ENV TERM xterm-256color
ENTRYPOINT ["/bin/bash", "--login", "-i"]

################################################################################
# Development with a user, e.g. for vscode devcontainers
################################################################################

ARG USERNAME=oxide
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -s "/bin/bash" -m $USERNAME && \
    echo "${USERNAME} ALL=NOPASSWD: ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}


RUN rustup component add clippy \
    && rustup component add rustfmt \
    && cargo install bindgen-cli --version 0.68.1 \
    && cargo install cargo-rdme cargo-expand

# The previous command re-roots permissions in ${CARGO_HOME} after the base image
# configured them to be wide. Re-establish wide permissions for zen.

RUN find ${CARGO_HOME} -type d -exec chmod 777 {} + && \
    find ${CARGO_HOME} -type f -exec chmod a+r {} +