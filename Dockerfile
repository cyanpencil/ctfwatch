# ubuntu 22 breaks podman
FROM alpine:3.17

RUN apk update
RUN apk add chafa cargo openssl1.1-compat-dev ca-certificates bash socat

COPY entrypoint.sh /entrypoint.sh

CMD ["/bin/bash", "entrypoint.sh"]


# later versions of archlinux have a broken pacman
#FROM archlinux:base-devel-20210131.0.14634 

#RUN yes Y | pacman -Syy && yes Y | pacman -S archlinux-keyring --noconfirm &&\
	#yes Y | pacman -Syyuu --noconfirm &&\
	#yes Y | pacman -S cargo socat gcc pkgconf openssl chafa --noconfirm

#COPY entrypoint.sh /entrypoint.sh

#CMD ["/bin/bash", "entrypoint.sh"]
