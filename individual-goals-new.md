# Individual goals

<!--
A description of the functionality that you intend to implement.
- A motivation for its value and relevance to LongGé.
- How you intend to implement it.
- How it shall be evaluated.
-->

1. Creating our own modified version of the Rust [nRF24l01+](https://crates.io/crates/nrf24l01) library. The existing Rust library does not seem to be functional, most likely because it is too outdated. But we would still prefer to use Rust in this project for a few reasons:

   - Rust would also run much faster than Python, while being easier to make memory- and runtimesafe than C

   - We both currently take the course in functional programming, where Haskell is used. So it would be nice to solidify own knowledge by using Rust in this course

   - We think that we could make the existing library more effective, by decreasing the address length used in the nRF24l01+ transciever from 5 bytes to 3 bytes

   This could would be evaluated by running the code. If it managed to transmit data between the transcievers, our implementation of the library works. To determine if the decrease in address length is successful, we could show the code determining the address length in the library and that the program still works with it.
