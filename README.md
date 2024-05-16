# EITN30 - Internet Inside

## Goal

The goal with the project is create an network interface where IP packets can be sent concurrently between the mobile unit and the base station, with Enhanced ShockBurst™ packets (see the datasheet):

```bash
ping –I longge –c 3 8.8.8.8
```

where:

- `-I <interface>` is either the interface name or address
- `-c <count>` stops after `<count>` replies

For our individual goals, see the [individual goals markdown file](/individual-goals.md).

## Quickstart

The following dependencies are needed:

- <https://www.gnu.org/software/make/>
- <https://github.com/cross-rs/cross>

And the following programs are needed:

- [Docker](https://www.docker.com/) (on the development machine)
- [Tailscale](https://tailscale.com/) (on both Pis and the development machine)

### Library

The library for the `nRF24l01+` network card is a modified version of [an existing Rust library](https://crates.io/crates/nrf24l01). The following commands should not be needed, however they can sometimes fix a lot of problems if the project is not build correctly:

```bash
cargo clean       # Clean the project

cd rust-nrf24l01  # Move to the library directory
cargo clean       # Clean the library

cd ..             # Move back to the root directory
cargo build       # Build the project
```

If that doesn't work either, the library may need to be build after the second `cargo clean` command.

### Build and deploy

The project code is written in Rust, and is built using `cross` for the `aarch64-unknown-linux-gnu` architecture. Make sure that docker is installed and running.

Copy the contents of `env.sh.example` in the `scripts` directory to a file called `env.sh` in the same directory, and fill in the IP addresses for deployment to base and mobile. Build and deploy the project by running the following commands from the root directory:

```bash
make build

make deploy-mobile  # To deploy only to the mobile unit
make deploy-base    # To deploy only to the base station
make deploy         # To deploy to both units
```

### SSH

For SSH to work both in the lab, and at home, we use [tailscale](https://tailscale.com/). To SSH into a Raspberry Pi, the correct SSH keys are needed. Then run:

```bash
make connect-mobile  # To connect to the mobile unit
make connect-base    # To connect to the base station
```

### PI setup

- Enable SPI bus 0 and 1 with 1 CS pin.
- Add a service file called `longge.service`, corresponding to the content from either the file `deploy/longge-base.service` or `deploy/longge-mobile.service`, depending on what unit is being configured. Then run:

```bash
sudo systemctl daemon-reload          # To read in the new service file
sudo systemctl enable longge.service  # To enable autostart
```

The service should now start automatically on boot. To start the service manually and check that it is working, you can run:

```bash
sudo systemctl start longge.service   # To start the service
sudo systemctl status longge.service  # To check the status
```

This will run the `eitn30-internet-inside` script from the `eitn30` directory. The script can also be tested manually by running:

```bash
cd eitn30        # To move to the eitn30 directory
make run-mobile  # On the mobile unit
make run-base    # On the base station
```

DNS must be configured correctly to use longge together with tailscale.

```bash
sudo nvim /etc/NetworkManager/NetworkManager.conf
```

Add the following line to the file:

```bash
dns=default
```

Then run the following commands:

```bash
sudo systemctl restart NetworkManager
sudo nvim /etc/resolv.conf
```

Add the following line to the file

```bash
nameserver 8.8.8.8
nameserver 8.8.4.4
```

Then reboot!

### Development

CI/CD is as simple as rebuilding and redeploying the code!

```bash
make build deploy
```

To monitor the network traffic on the longge interface, run the following commands of your choice on one of the PIs:

```bash
sudo tcpdump -i longge  # add dst 10.0.0.<transmitter_address> to see only received packages, and src 10.0.0.<receiver_address> to see only sent packages
sudo iptables -S        # to see the iptables rules
ip addr                 # to show all interfaces
netstat -r              # to show the routing table
```

## Devices

The following components are used in the project:

- 1 Raspberry Pi 5 and 1 Raspberry Pi 4 ([Pinout](https://pinout.xyz/))
- nRF24L01+ 2.4 GHz Transciever ([Datasheet](https://www.sparkfun.com/datasheets/Components/SMD/nRF24L01Pluss_Preliminary_Product_Specification_v1_0.pdf))

The transcievers are connected to each Pi as follows:

<center>

| inuti24 (Mobile) | SPI bus | SPI device | Device Number | CE GPIO | Position (relative to `inuti24` text) |
| ---------------- | ------- | ---------- | ------------- | ------- | ------------------------------------- |
| Transmitter      | 0       | 0          | 1             | 7       | Top                                   |
| Receiver         | 1       | 0          | 2             | 17      | Bottom                                |

| inuti32 (Base) | SPI bus | SPI device | Device Number | CE GPIO | Position (relative to `inuti24` text) |
| -------------- | ------- | ---------- | ------------- | ------- | ------------------------------------- |
| Transmitter    | 0       | 0          | 1             | 17      | Top                                   |
| Receiver       | 1       | 0          | 2             | 7       | Bottom                                |

</center>
