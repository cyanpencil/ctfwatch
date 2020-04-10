FROM archlinux

RUN pacman -Syy && \
	pacman -S cargo socat gcc pkgconf openssl chafa --noconfirm

COPY entrypoint.sh /entrypoint.sh

CMD ["/bin/bash", "entrypoint.sh"]
