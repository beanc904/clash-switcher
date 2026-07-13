FROM ubuntu:noble

ARG PKG_NAME="Clash.Verge_2.5.1_amd64.deb"
# ARG PKG_NAME="Clash.Verge_2.5.1_arm64.deb"

WORKDIR /workspace

ADD https://github.com/beanc904/clash-verge-rev/releases/download/v2.5.1-se/${PKG_NAME} .
# ADD https://github.com/clash-verge-rev/clash-verge-rev/releases/download/v2.5.1/${PKG_NAME} .

VOLUME [ "/root/.local/share/io.github.clash-verge-rev.clash-verge-rev", "/root/.config/io.github.clash-verge-rev.clash-verge-rev" ]

RUN apt update && apt upgrade -y \
    && apt install -y openssl libayatana-appindicator3-1 libwebkit2gtk-4.1-0 libgtk-3-0 \
    && dpkg -i ${PKG_NAME} \
    && apt install -y locales fonts-wqy-zenhei fonts-wqy-microhei \
    && locale-gen zh_CN.UTF-8 \
    && rm ${PKG_NAME} \
    && apt clean && rm -rf /var/lib/apt/lists/*

ENTRYPOINT [ "/usr/bin/clash-verge" ]
