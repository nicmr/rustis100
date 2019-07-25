# Rustis-100

A TIS-100 parser and emulator written in rust.

TIS-100 is a fictive highly parallel computer architecture and instruction set by Zachtronics, LLC, featured in their identically titled game.
If this project sparks your interest, please [consider buying the game](https://www.gog.com/game/tis100).

## Architecture and instruction set

The official architecture and instruction set are property of Zachtronics LLC.

Please have a look at the manual on their website:
[TIS-100 Manual](http://www.zachtronics.com/images/TIS-100P%20Reference%20Manual.pdf)

## Usage
```zsh
git clone https://github.com/nicmr/rustis100
cargo run
```

## Concept

Generally, the work rustis-100 does under the hood can be split into three different tasks

1. Parse all instructions
1. Emulating a TIS-100 and running the instructions

Let's take a closer look at how we can perform these tasks more efficiently by parallelizing our code.


## Parsing

Parsing TIS-100 instructions is pretty staightforward.
There's only 12 operators with a fixed number of parameters each.

 - Code of different nodes can be parsed in parellel without any issues, as instructions of one node never care about the context of other nodes.
 - Parsing all instructions in a single node parallel is a bit more difficult, as jump instructions `JEZ`, `JLZ`, `JGZ` need to know about the position of jumpmarks. 
 We're adressing this by replacing them with `JRO` (jump by offset) instructions in an initial parsing step.


## Emulator

The most simple way to emulate the TIS-100 is sequentially running the current instruction of all nodes.
Parallelizing the emulator is more difficult to achieve as the nodes all have to run at a single, synchronous clock rate.

There are however some tricks we can use to process at least some parts of the system in parallel.

### 1.  Vector clock / Lamport timestamp
**Description:**
Parallelize nodes that take no or minimal input at the beginning of their run but produce a lot of outputs.
Instead of blocking and waiting for their generated output to be consumed, the result is stored together with a timestamp of their current tick.
Three possible cases can occur when the consumer attempts to consume the oldest input from the channel:
 1. tick in channel == tick of consumer: In this rare case the program can keep running without any adjustments
 2. tick in channel < tick of consumer: If the interval between the consumers internal time and the tick at which the message was stored is greater than the interval between the messages tick and the next messages tick, all following messages will need to have their timestamps adjusted to simulate a write-block on the unbuffered channel we're emulating.
 3. tick in channel > tick of consumer: The consumer will have to update its internal time sto the tick stored in the channel to simulate blocking.
 If the interval until the next entry in the channel is. Then the same interval check as in (2) will have to be performed.

**`ANY:`** This feature has some complicated interactions with the `ANY` instrcution:
A channel that can be read from with `ANY` may not have any information in it yet because the writing node has yet to produce said information.
The following algorithm can resolve the issue:

```
WHILE (not all channels have at least one available message)
    DO (check channels)
read the oldest message
update own tick timestamp to emulate blocking
```
This might, however, result in a deadlock - input on one of the inputs for `ANY` may depend on an output the node at hand still has to create.
Further research is required.