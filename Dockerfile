# To run in this Docker container on an x86-64 ISA, this must be built using
# the MUSL/Linux target. It must also be built on a Unix-like system.
#
# To install this target, run: `rustup target add x86_64-unknown-linux-musl`.
#
# You must compile the program first by running
# cargo build --release --target x86_64-unknown-linux-musl --bin main
FROM alpine
LABEL author="Jonathan M. Wilbur <jonathan@wilbur.space>"
LABEL environment="production"
COPY ./i18n /srv/i18n
COPY ./target/x86_64-unknown-linux-musl/release/main /srv
WORKDIR /srv
RUN chmod +x /srv/main
CMD [ "/srv/main" ]
