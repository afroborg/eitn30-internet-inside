## Theoretical performance

### Maximal throughput

One byte is transmitted every `delay` microseconds. With a delay of 20 $\mu s$, the maximum possible bitrate is:

$$\frac{1 B}{20 \mu s} = \frac{8 b}{2 \cdot 10^{-5} s} = 400 kbps$$

The nRF24L01+ has a maximum bitrate of 2 Mbps, so the delay could theoretically be decreased to 2.5 $\mu s$ to achieve this. 

The Enhanced ShockBurst™ packet consists of:
- A header containing a 1 B preamble, 3-5 B address (3 B with our library) and a 9 b packet control field (to simplify calculations, this is assumed to be 1 B)
- A payload of 0-32 B
- A trailer of 1-2 B CRC (2 B with our library)

The minimum packet size for the largest payload with our library is therefore:

$$1B + 3B + 1B + 32B + 2B = 39B$$

Which means that the maximum possible throughput is:

$$400 kbps \cdot \frac{32B}{39B} = 328 kbps$$

### Other considerations

A packet in the TX FIFO queue is transmitted after certain bits have kept their values for $10 \mu s$. <!-- Continue with part 7.8 in the datasheet -->

## Testing

Testing using iPerf3:

| Try #    | Bitrate (Kbps) | Change                                                  |
| -------- | -------------- | ------------------------------------------------------- |
| 1        | 290            | -                                                       |
| 2        | 290            | Decrease transmitter retry delay from 2250 us to 750 us |