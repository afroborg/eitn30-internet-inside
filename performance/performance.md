## Maximal throughput

### Packet structure

An _Enhanced ShockBurst™_ packet consists of:

- A header with:
  - 1 B preamble
  - 3-5 B address (3 B with our library)
  - 9 b packet control field
- A payload of 0-32 B
- A trailer of 1-2 B CRC (2 B with our library)

The minimum combined header and trailer length with our library is therefore $1B + 3B + 9b + 2B = 57b$. For the largest payload of 32 B we therefore get a packet length of:

$$57b + 32B = 313b$$

### Total time per packet

The time to transmit an entire _Enhanced ShockBurst™_ packet is $T_{ESB}$, which - when considering ACKs - is calculated as:

$$T_{ESB} = T_{UL} + T_{stdby2a} + T_{OA} + T_{ACK} + T_{IRQ}$$

Where:

- $T_{UL}$ is the time to upload the payload through the SPI link
- $T_{stdby2a}$ is the time to transition from _Standby-I_ to _Active_ mode
- $T_{OA}$ is the time-on-air, the time to transmit a packet
- $T_{ACK}$ is the time to transmit an ACK packet

The datasheet provides a max SPI speed of 10 Mbps, which means that the minimum $T_{UL}$ is:

$$T_{UL} = \frac{313b}{10 \cdot 10^6 \frac{b}{s}} = 31.3 \mu s$$

The time to transition from _Standby-I_ to _Active_ mode is also given as $T_{stdby2a} = 130 \mu s$.

The time on air depends on the bitrate of the transceiver. The nRF24L01+ has a maximum bitrate of 2 Mbps, which means one bit can be transmitted every 0.5 $\mu s$, or one byte every 4 $\mu s$. We then get the $T_{OA}$ as:

$$T_{OA} = 313b \cdot 0.5 \mu s = 156.5 \mu s$$

The time for the ACK, with a payload, would be the same as the $T_{OA}$, so:

$$T_{ACK} = T_{OA} = 152.5 \mu s$$

The total time to transmit a packet is therefore:

$$T_{ESB} = 30.5 \mu s + 130 \mu s + 152.5 \mu s + 152.5 \mu s = 465.5 \mu s$$

<!--
However, the delay between starting to transmit a packet, and starting to read the received ACK, must be at least $360 \mu s$, which is reflected in the `send` function in the library. Currently, this delay is only $152.5 \mu s$, which is too short. Therefore, an additional delay of $360 \mu s - 152.5 \mu s = 207.5 \mu s$ can be added to the total transmit time:

$$T_{ESB\_Final} = 465.5 \mu s + 207.5 \mu s = 673 \mu s$$
-->

### Throughput

The maximum theoretical bitrate can be calculated by first dividing the packet length by the total delay:

$$\frac{313b}{465.5 \mu s} = \frac{313b}{465.5 \cdot 10^{-6} s} = 672.4 kbps$$

Of which, only 32B is payload. This means that the maximum theoretical throughput is:

$$\frac{32B}{465.5 \mu s} = \frac{256 b}{465.5 \cdot 10^{-6} s} = 550.0 kbps$$

## Proposed changes

### ACK

We tried to disable automatic ACK in the library, with the idea that the throughput could be increased if the receiver would not need to transmit an ACK for each received package.

However, to the extent that we got this working, it did not seem to have any effect on the throughput. And according to the datasheet, the dynamic payload length would also need to be disabled for the ACK to be disabled. When trying this, the transmission did not work at all. Our theory is that the library is dependent on ACKs being sent, so removing ACKs would require the rewriting of most of the library. Therefore we instead reverted these changes.

### CRC

We also tried to decrease the CRC to 1 B, as the datasheet states that this should be possible. However, changing this made the program non-functional. The same thing occured when removing the CRC altogether. Therefore, we reverted these changes as well.
