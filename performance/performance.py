import os
import sys
import iperf3
import datetime
import json
import time

def restart_service():
    print("Restarting longge service\n")
    os.popen("sudo systemctl restart longge.service")

def parse_bandwidth(bandwidth: str) -> list:
    """
    Parse the bandwidth argument passed to the script.
    """

    list_b = bandwidth.split(':')

    [start_b, step_b, end_b] = [0, 0, 0]
    if len(list_b) == 1:
        [start_b, step_b, end_b] = [int(bandwidth), -1, int(bandwidth)]
    elif len(list_b) == 3:
        [start_b, step_b, end_b] = map(int, list_b)
    else:
        print(f'Invalid bandwidth argument: {sys.argv[2]}')
        sys.exit(1)

    return map(lambda x: x * 1000, [start_b, step_b, end_b])

def read_arguments():
    """
    Read the arguments passed to the script.
    """

    [server_ip, server_port] = sys.argv[1].split(':')
    [start_b, step_b, end_b] = parse_bandwidth(sys.argv[2])

    duration = int(sys.argv[3])
    protocol = sys.argv[4] if len(sys.argv) > 4 else 'udp'
    
    return (server_ip, start_b, step_b, end_b, server_port, duration, protocol)

def create_client(server_ip: str, server_port: str, bandwidth: int, duration: int, protocol: str) -> iperf3.Client:
    """
    Create an iperf3 client object.

    Returns:
    client: The iperf3 client object.
    """

    # iperf3 -c <server_ip> -u -b 100K -l 32 -t 60
    client = iperf3.Client()                # -c, --client <arg>    run in client mode, connecting to <host>
    client.bind_address = '10.0.0.1'
    client.server_hostname = server_ip      # <server_ip>
    client.port = server_port               # -p, --port <arg>      server port to connect to
    client.protocol = protocol              # -u, --udp             use UDP rather than TCP
    client.bandwidth = bandwidth            # -b, --bandwidth <arg> for UDP, bandwidth to send at in bits/sec
    client.blksize = 32                     # -l, --len <arg>       set length read/write buffer size
    client.duration = duration              # -t, --time <arg>      time in seconds to transmit for (default 10 secs)

    return client

def save_results(bandwidth: int, protocol: str, result: json):
    """
    Save the results of the performance tests to a file.
    """

    date_now = datetime.datetime.now().strftime('%Y-%m-%dT%H:%M:%S')

    output_file = open(f"data/{protocol}/{bandwidth // 1000}K-{date_now}.json", "w+")
    output_file.write(json.dumps(result, indent=2))
    output_file.close()

def extract_results(result: json):
    """
    Extract the results of the performance test from the iperf3 result object.
    """

    sum = result["end"]["sum"]

    return {
        'seconds': sum['seconds'],
        'bits_per_second': sum['bits_per_second'],
        'jitter_ms': sum['jitter_ms'],
        'lost_packets': sum['lost_packets'],
        'packets': sum['packets'],
    }


def test_performance(client: iperf3.Client):
    """
    Perform a performance test using iperf3.
    """

    print(f'')
    print(f'Transmitting at {client.bandwidth // 1000} kbps for {client.duration} seconds')
    result = client.run()

    if result.error:
        print(result.error)
        return
    
    print(f'  the test took      {result.seconds} seconds')
    print(f'  bandwidth (kbps)   {result.bps}')
    print(f'  jitter (ms)        {result.jitter_ms}')
    print(f'  lost packets (%)   {result.lost_percent}')


    return extract_results(result.json)

def test_performances():
    """
    Perform a series of performance tests using iperf3.
    """

    (server_ip, start_b, step_b, end_b, server_port, duration, protocol) = read_arguments()

    print(f'Testing performance for {protocol} protocol')
    print(f'  server: {server_ip}:{server_port}')

    if step_b > 0:
        print(f'  testing bandwidths from {start_b // 1000} kbps to {end_b // 1000} kbps in steps of {step_b // 1000} kbps')

    results = []

    for bandwidth in range(start_b, end_b + 1, abs(step_b)):
        print(f'')
        print(f'Testing bandwidth {bandwidth // 1000} kbps')

        # Perform a test for each bandwidth and duration        
        client = create_client(server_ip, server_port, bandwidth, duration, protocol)

        results.append({
            'bandwidth': bandwidth,
            **test_performance(client)
        })

        del client
        
        time.sleep(1)
    
    save_results(bandwidth, protocol, { 'server': f'{server_ip}:{server_port}', 'duration': duration , 'result': results })

    print(f'')
    print(f'Performance test completed')

if __name__ == '__main__':
    test_performances()