FROM ubuntu:noble

ARG PKG_NAME="Clash.Verge_2.5.1_amd64.deb"
WORKDIR /workspace

ENV DBUS_SESSION_BUS_ADDRESS=unix:path=/var/run/user/dbus.sock \
    LANG=zh_CN.UTF-8 \
    LANGUAGE=zh_CN:zh

VOLUME [ "/root/.local/share/io.github.clash-verge-rev.clash-verge-rev", "/root/.config/io.github.clash-verge-rev.clash-verge-rev" ]

RUN apt update && apt install -y --no-install-recommends \
    curl \
    ca-certificates \
    dbus \
    openssl \
    libayatana-appindicator3-1 \
    libwebkit2gtk-4.1-0 \
    libgtk-3-0 \
    locales \
    fonts-wqy-zenhei \
    fonts-wqy-microhei \
    && locale-gen zh_CN.UTF-8 \
    && dbus-uuidgen --ensure \
    \
    && curl -L -O "https://github.com/beanc904/clash-verge-rev/releases/download/v2.5.1-se/${PKG_NAME}" \
    && dpkg -i ${PKG_NAME} \
    && rm ${PKG_NAME} \
    \
    && apt purge -y curl \
    && apt autoremove -y \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*

ENV LC_ALL=zh_CN.UTF-8 \
    NO_AT_BRIDGE=1

ENTRYPOINT ["/bin/sh", "-c", "dbus-daemon --session --address=$DBUS_SESSION_BUS_ADDRESS --nofork & exec /usr/bin/clash-verge \"$@\"", "--"]
