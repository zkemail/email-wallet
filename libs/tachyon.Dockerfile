FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive

# Install required dependencies
RUN apt-get update && \
  apt-get install -y curl gnupg git libgmp-dev libomp-dev python3 python3-pip zip


RUN pip install numpy

# Install Bazel
RUN curl -fsSL https://bazel.build/bazel-release.pub.gpg | gpg --dearmor > bazel.gpg && \
  mv bazel.gpg /etc/apt/trusted.gpg.d/ && \
  echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" | tee /etc/apt/sources.list.d/bazel.list && \
  apt-get update && \
  apt-get install -y bazel-6.3.0 && \
  ln -s /usr/bin/bazel-6.3.0 /usr/local/bin/bazel

# Set Python 3 as the default python
RUN ln -s /usr/bin/python3 /usr/bin/python

# Clone and build Tachyon
RUN git clone --branch v0.3.0 https://github.com/kroma-network/tachyon /tachyon
WORKDIR /tachyon/vendors/circom
RUN CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index && bazel build --@kroma_network_tachyon//:has_openmp -c opt --config linux //:prover_main

ENTRYPOINT ["/tachyon/vendors/circom/bazel-bin/prover_main"]