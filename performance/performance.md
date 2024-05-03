## Maximal throughput

### Packet structure

An _Enhanced ShockBurst™_ packet consists of:
- A header with:
    - 1 B preamble
    - 3-5 B address (3 B with our library)
    - 9 b packet control field
- A payload of 0-32 B
- A trailer of 1-2 B CRC (1 B with our library)

The minimum combined header and trailer length with our library is therefore $1B + 3B + 9b + 1B = 49b$. For the largest payload of 32 B we therefore get a packet length of:

$$49b + 32B = 305b$$

### Total time per packet

The time to transmit an entire _Enhanced ShockBurst™_ packet is $T_{ESB}$, which - when considering ACKs - is calculated as:

$$T_{ESB} = T_{UL} + 2 \cdot T_{HCE} + 2 \cdot T_{stdby2a} + T_{OA} + T_{ACK} + T_{IRQ}$$

Where:
- $T_{UL}$ is the time to upload the payload through the SPI link
- $T_{HCE}$ is the time that the CE pin is high before the transmitter is activated
- $T_{stdby2a}$ is the time to transition from _Standby-I_ to _Active_ mode
- $T_{OA}$ is the time-on-air, the time to transmit a packet
- $T_{ACK}$ is the time to transmit an ACK packet

The datasheet provides a max SPI speed of 10 Mbps, which means that the minimum $T_{UL}$ is:

$$T_{UL} = \frac{305b}{10 \cdot 10^6 \frac{b}{s}} = 30.5 \mu s$$

The time for $T_{HCE}$ is given by the datasheet as $10 \mu s$. The time to transition from _Standby-I_ to _Active_ mode is also given as $T_{stdby2a} = 130 \mu s$.

The time on air depends on the bitrate of the transceiver. The nRF24L01+ has a maximum bitrate of 2 Mbps, which means one bit can be transmitted every 0.5 $\mu s$, or one byte every 4 $\mu s$. We then get the $T_{OA}$ as:

$$T_{OA} = 305b \cdot 0.5 \mu s = 152.5 \mu s$$

The time for the ACK, with a payload, would be the same as the $T_{OA}$, so:

$$T_{ACK} = T_{OA} = 152.5 \mu s$$

The total time to transmit a packet is therefore:

$$T_{ESB} = 30.5 \mu s + 2 \cdot 10 \mu s + 2 \cdot 130 \mu s + 152.5 \mu s + 152.5 \mu s = 615.5 \mu s$$

### Throughput

The theoretical bitrate can be calculated by first dividing the packet length by the total delay:

$$\frac{305b}{615.5 \mu s} = \frac{305 b}{615.5 \cdot 10^{-6} s} = 495.9 kbps$$

Of which, only 32B is payload. This means that the maximum theoretical throughput is:

$$\frac{32B}{615.5 \mu s} = \frac{256 b}{615.5 \cdot 10^{-6} s} = 416.1 kbps$$

## Other considerations

A packet in the TX FIFO queue is transmitted after certain bits have kept their values for $10 \mu s$. <!-- Continue with part 7.8 in the datasheet -->

## ARQ

We tried to disable automatic ACK in the library, with the idea that the throughput could be increased if the receiver would not need to transmit an ACK for each received package.

However, to the extent that we got this working, it did not seem to have any effect on the throughput. And according to the datasheet, the dynamic payload length would also need to be disabled for the ACK to be disabled. When trying this, the transmission did not work at all. Our theory is that the library is dependent on ACKs being sent, so removing ACKs would require the rewriting of most of the library. Therefore we instead reverted these changes.

## Testing

### MTU

| MTU (B)  | Observed transmission speed |
| -------- | --------------------------- |
| 1500     | Slow                        |
| 65535    | Fast                        |

### Delay

| Delay (us) | Observed transmission speed    |
| ---------- | ------------------------------ |
| 5          | Fast                           |
| 20         | Fast                           |
| 100        | Slow                           |
| 320        | About halved from a 5 us delay |

### PACKET_MAX_RETRIES

| MAX_NBR_RETRIES | Observed transmission speed |
| --------------- | --------------------------- |
| 2               | Fast                        |
| 15              | Fast                        |

### PACKET_RETRY_DELAY

| PACKET_RETRY_DELAY | Observed transmission speed |
| ------------------ | --------------------------- |
| 1                  | Fast                        |
| 2                  | Fast                        |
| 10                 | Fast                        |
