# 🏓 Kubernetes Exercise

This repo contains exercise for learning kubernetes.
The goal is to have 3 pods, where 1 pod is talking to the other 2 pods using internal service name.

Here's the diagram:

                   +---------------------+
                   |      Browser        |
                   +--------+---+--------+
                              |
                              |
                              |
                   +----------v----------+
                   |     pingpong.rs     |
                   +--------+---+--------+
                            |   |
                            |   |
                            |   |
                            |   |
    +-----------------+     |   |    +------------------+
    |     ping.rs     <-----+   +---->     pong.rs      |
    +-----------------+              +------------------+
