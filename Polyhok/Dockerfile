FROM nvidia/cuda:12.6.0-devel-ubuntu24.04

ENV DEBIAN_FRONTEND=noninteractive

# =========================
# System deps
# =========================
RUN apt-get update && apt-get install -y \
    git \
    curl \
    wget \
    ca-certificates \
    gnupg \
    build-essential \
    autoconf \
    libssl-dev \
    libncurses5-dev \
    libncurses-dev \
    libwxgtk3.2-dev \
    libwxgtk-webview3.2-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libpng-dev \
    libssh-dev \
    unixodbc-dev \
    xsltproc \
    fop \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# =========================
# Erlang/Elixir repo (CORRECT WAY)
# =========================
RUN wget -O- https://packages.erlang-solutions.com/ubuntu/erlang_solutions.asc \
    | gpg --dearmor > /usr/share/keyrings/erlang.gpg

RUN echo "deb [signed-by=/usr/share/keyrings/erlang.gpg] https://packages.erlang-solutions.com/ubuntu noble contrib" \
    > /etc/apt/sources.list.d/erlang.list

# =========================
# Install Erlang 27 + Elixir 1.16
# =========================
RUN apt-get update && apt-get install -y \
    esl-erlang \
    elixir \
    && rm -rf /var/lib/apt/lists/*

# =========================
# Verify versions
# =========================
RUN erl -eval 'erlang:display(erlang:system_info(otp_release)), halt().' -noshell
RUN elixir --version
RUN nvcc --version

# =========================
# Workspace
# =========================
WORKDIR /workspace

RUN git clone https://github.com/ardubois/poly_hok.git

WORKDIR /workspace/poly_hok

# =========================
# Mix setup
# =========================
RUN mix local.hex --force && \
    mix local.rebar --force && \
    mix deps.get || true

CMD ["/bin/bash"]
