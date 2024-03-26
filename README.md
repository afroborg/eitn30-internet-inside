# EITN30 - Internet Inside

## Components

The following components are used in the project:

- Raspberry Pi 5 ([Pinout](https://pinout.xyz/))
- nRF24L01+ 2.4 GHz Transciever ([Datasheet](https://www.sparkfun.com/datasheets/Components/SMD/nRF24L01Pluss_Preliminary_Product_Specification_v1_0.pdf))

## Goal

The goal with the project is create an network interface where IP packets can be sent concurrently between the mobile unit and the base station, with Enhanced ShockBurst™ packets (see the datasheet):

```bash
ping –I MyG –c 3 8.8.8.8
```

<!-- TODO: Change from MyG to whatever we call our interface -->

where:

- `-I <interface>` is either the interface name or address
- `-c <count>` stops after `<count>` replies

## Quickstart

The following dependencies are needed:

- <https://github.com/cross-rs/cross>
- <https://github.com/meh/rust-tun>
- <https://www.gnu.org/software/make/>

First, build the library for the `nRF24l01` network card:

```bash
cd rust-nrf24l01  # Move to the library directory
cargo build       # Build the library
```

The project is built using `cross` for the `aarch64-unknown-linux-gnu` with the following shell script. This requires that docker is installed and running.

```bash
make build
```

<!-- TODO: Update the deploy description when a more general deploy script is created -->

To deploy the project to the `inutiXX` Pi in the lab, run the following shell script:

```bash
make deploy-mobile  # To deploy to the mobile unit
make deploy-base    # To deploy to the base station
```

SSH into a Raspberry Pi by running:

```bash
make connect-mobile  # To connect to the mobile unit
make connect-base    # To connect to the base station
```

And run the script from the `eitn30` directory:

<!-- TODO: Change script when both PIs communicate with eachother -->

```bash
cd eitn30        # To move to the eitn30 directory
make run-mobile  # On the mobile unit
make run-base    # On the base station
```

### Development

When the script is running, the network information can be viewed by running:

```bash
sudo tcpdump -i longge # add dst 10.0.0.<transmitter_address> to see only received packages, and src 10.0.0.<receiver_address> to see only sent packages
```

## Devices

<center>

| Device       | SPI bus | SPI device | Device Number | CE GPIO | Position (relative to `inutixx` text) |
|--------------|---------|------------|---------------|---------|---------------------------------------|
| Transmitter  | 0       | 0          | 1             | 7       | Top                                   |
| Receiver     | 1       | 0          | 2             | 17      | Bottom                                |

</center>

## TODO?
