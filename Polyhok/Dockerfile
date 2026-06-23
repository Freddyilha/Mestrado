FROM nvidia/cuda:12.6.0-devel-ubuntu24.04

ENV DEBIAN_FRONTEND=noninteractive

# =========================
# System dependencies (FIXED for Ubuntu 24.04)
# =========================
RUN apt-get update && apt-get install -y \
    git curl wget ca-certificates \
    build-essential autoconf automake \
    libssl-dev \
    libncurses-dev \
    libreadline-dev \
    libwxgtk-webview3.2-dev \
    libgl1-mesa-dev libglu1-mesa-dev \
    unixodbc-dev xsltproc fop \
    unzip \
    gfortran \
    libatlas-base-dev \
    libopenblas-dev \
    liblapack-dev \
    && rm -rf /var/lib/apt/lists/*

# =========================
# Erlang + Elixir (no ASDF, stable official packages)
# =========================
RUN apt-get update && apt-get install -y \
    erlang \
    erlang-dev \
    elixir \
    && rm -rf /var/lib/apt/lists/*

# =========================
# Workspace
# =========================
WORKDIR /workspace

CMD ["/bin/bash"]
