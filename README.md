# Emergency in space

![Gophers floating in space](img/space-gophers.webp)

Welcome to the Situation Room, _YOUR NAME_. Thank you for coming in at such short notice. Eleven hours and sixteen minutes ago, our scientists lost contact with the Trailblazer 1 probe, en route to Proxima Centauri. Since then we've been trying without success to re-establish contact with the spacecraft's onboard computer. Unless the fault can be rectified, and soon, we are facing a total loss of the vehicle and mission.

The problem appears to be localised to a subsystem known as the _RTFM_ (Remote Telemetry and Flight Management) computer, responsible for the communications link between the spacecraft and Earth. Engineers are speculating that its memory may have been corrupted by a cosmic ray impact, causing the software to malfunction.

That's where you come in. We urgently need your help to investigate the RTFM software issue. Specifically, the first thing we need is a Go program that can execute code written for the spacecraft's onboard computer. A program like this is called an **emulator**: it acts like a model of the computer's central processing unit (**CPU**), and will enable us to write and test code before uploading it to the RTFM for real.

You might also have heard the term **virtual machine**, meaning not “nearly a machine”, but something more like “not physically a machine, but behaving in the same way”. Emulator, virtual machine, same thing.

It may sound complicated, but emulators are actually fairly easy to write, once you understand how the emulated machine (the **guest**) works. The RTFM computer uses a relatively small and simple CPU, compared to the one in your computer (the **host** machine).

This CPU is called the **R8**, and to emulate it, you'll be writing the necessary Go code to implement the R8's **instruction set**: the commands it understands.

The first thing the emulator needs is a properly-initialised CPU, ready to work. So you can call `Cpu::default()` to get a CPU in its default initial state, which is specified by a test.

The `Cpu` struct and its test are already implemented in the [`src/lib.rs`](src/lib.rs) file, so your first challenge is a straightforward one:

**GOAL:** Run the test with `cargo test`, and make sure it passes.

That's a great start! For the next challenge, read the tutorial:

* [Welcome to the machine: emulating a CPU](https://bitfieldconsulting.com/posts/welcome-to-machine)
