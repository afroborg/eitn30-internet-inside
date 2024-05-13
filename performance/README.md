# Performance testing

## Mobile unit (client)

Create a venv and install the requirements:

```bash
python3 -m venv venv
source venv/bin/activate
python3 -m pip install -r requirements.txt
```

### Run tests

Run the tests by running the `performance.py` script. The script takes the following arguments:

- `<server_ip:server_port>`: The IP and port of the server to connect to.
- `<bandwidth>`: The bandwidth to test, in kbps.
- `<duration>`: The duration of the test in seconds.
- `<protocol>`: The protocol to use. Can be either `tcp` or `udp`. Defaults to `udp`.

Example usage:

```bash
python performance.py 10.0.0.0:5002 10 5 # Tests 10 kbps for 5 seconds using the UDP protocol.
```

Alternatively, use the iperf3 command in the terminal, but some magic is required to get a good JSON output:

```bash
iperf3 -c 10.0.0.0 -p 5002 -b <bandwidth> -t <duration> -u
```

### Plot data

When copying over the test data from the mobile unit to the current terminal directory, you can use the following command. It is recommended to run this from the `/performance` directory:

```bash
scp -r -i ~/.ssh/eitn30-pi pi@<mobile_ip>:~/eitn30/data .
```

Plot the data using the `plot.py` script. The script takes one argument, the protocol to plot for (udp or tcp). The script will plot all the data in the `data/<protocol>` directory.

```bash
python plot.py <protocol>
```

## Base station (server)

On the base station, run the iperf3 server to allow the client to connect:

```bash
iperf3 -s -p <server_port>
```

Example usage:

```bash
iperf3 -s -p 5002
```
