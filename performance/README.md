# Performance testing

## Mobile unit (client)

Create a venv and install the requirements:

```bash
python3 -m venv venv
source venv/bin/activate
python3 -m pip install -r requirements.txt
```

Run the tests by running the `performance.py` script. The script takes the following arguments:

- `<server_ip:server_port>`: The IP and port of the server to connect to.
- `<start_bandwidth:step_bandwidth:end_bandwidth>`: The bandwidths to test. The script will test the bandwidths from `start_bandwidth` to `end_bandwidth` in steps of `step_bandwidth`.
- `<duration>`: The duration of the test in seconds.
- `<protocol>`: The protocol to use. Can be either `tcp` or `udp`. Defaults to `udp`.

Example usage:

```bash
python performance.py 10.0.0.0:5002 10:10:300 3 # Tests all bandwidths from 10 to 300 in steps of 10 for 3 seconds using the UDP protocol.
```

Alternatively, use the iperf3 command in the terminal to only run once:

```bash
iperf3 -c 10.0.0.0 -p 5002 -b <bandwidth> -t <duration> -u
```

## Base station (server)

On the base station, run iperf3 server:

```bash
iperf3 -s -p <server_port>
```

Example usage:

```bash
iperf3 -s -p 5002
```
