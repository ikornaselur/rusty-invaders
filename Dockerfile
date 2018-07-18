FROM rust:1

RUN apt-get update
RUN apt-get upgrade
RUN apt-get install -y libsfml-dev libcsfml-dev
