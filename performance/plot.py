import sys
import matplotlib.pyplot as plt
import json
import os

ALLOWED_PROTOCOLS = ['tcp', 'udp']

def read_json_file(file_path: str):
    """
    Read a json file and return its content.
    """
    with open(file_path, 'r') as file:
        return json.load(file)

def read_all_files(protocol: str) -> list:
    """
    Reads all the files in the directory for a specific protocol and returns the content of each file.
    """
    current_path = os.path.dirname(__file__)
    full_path = os.path.join(current_path, 'data', protocol)
    files = os.listdir(full_path)

    all_data = []

    for file in files:
        if file.endswith('.json'):
            file_data = read_json_file(os.path.join(full_path, file))
            all_data.append(file_data)
    
    return all_data

def parse(data: list) -> dict:
    """
    Parse the data from the performance tests and return the results as bandwidth and latency.
    """
    results = []

    for test in data:
        duration = test['duration']
        result = test['result']
        results.append({
            'bandwidth': result['bandwidth'],
            'latency': result['seconds'] - duration
        })
    
    sorted_results = sorted(results, key=lambda x: x['bandwidth'])
    
    return sorted_results

def plot_results(data: list):
    """
    Plot the results of the performance tests.
    """
    
    bandwidths = [x['bandwidth'] // 1000 for x in data]
    latencies = [x['latency'] for x in data]

    plt.plot(bandwidths, latencies)
    plt.title('Bandwidth vs Latency')

    plt.xlabel('Bandwidth (kbps)')
    plt.ylabel('Latency (s)')
    # plt.ylim(0, 10)

    plt.grid()
    
    plt.savefig('latency.png')

if __name__ == '__main__':
    protocol = sys.argv[1]

    if protocol not in ALLOWED_PROTOCOLS:
        print('Invalid protocol, allowed protocols are: ', ('\t').join(ALLOWED_PROTOCOLS))
        exit()

    data = read_all_files(protocol)
    parsed_data = parse(data)
    plot_results(parsed_data)