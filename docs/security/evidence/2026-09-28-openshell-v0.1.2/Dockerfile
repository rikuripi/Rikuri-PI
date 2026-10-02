FROM python@sha256:392307d22300de8b5986851a12d9176dfc0fc073e65bf6523ebd7dcbeb23564e
RUN mkdir -p /workspace /readonly /forbidden /test && chmod 777 /workspace /readonly /forbidden && printf "synthetic-visible\n" > /readonly/visible.txt && chmod 666 /readonly/visible.txt && printf "synthetic-forbidden\n" > /forbidden/not-for-agent.txt && chmod 644 /forbidden/not-for-agent.txt
COPY probe.py /test/probe.py
USER 1000:1000
WORKDIR /workspace
