Qemu Rust
=========

This code parses the Qemu QAPI and uses that to generate bindings.  This is
possible because Qemu is a Json based API and allows for easy language bindings.
If you would like to create bindings for a new language that isn't covered here
just add a template to the templates folder and follow the examples in there.

Unit Testing
------------
This is copied from (qemu-java)[https://github.com/shevek/qemu-java] because I think it hits at the heart of why this
project is important:

> It's all about development speed, and development speed is so much about test speed, and this package is a key part of the solution to test speed.

> A primary limiting factor on development speed is the elapsed time between writing code and obtaining test results. If the project being developed requires a full Unix environment, it is common to have to wait for an overnight Jenkins build, which can take hours, and is managed by a pile of ad-hoc shell scripts - yes, I know you've done it because I've done it. We've all done it.

> In addition, such ad-hoc shell scripts make it impossible to comprehensively test failure modes: disk, CPU, network, rack, port, since they cannot control the emulated environment in a fine grained manner. This package allows the automation of the assembly and manipulation of complex network topologies, control of the full lifecycle of the application and underlying VM, and the programmatic introduction of any fault which can be emulated by QEmu or Linux.
