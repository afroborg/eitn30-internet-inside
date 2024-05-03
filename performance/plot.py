import sys
import matplotlib.pyplot as plt
import numpy as np
import json

def read_json_file(file_path: str):
    """
    Read a json file and return its content.
    """
    with open(file_path, 'r') as file:
        return json.load(file)

def plot_results(data: dict):
    """
    Plot the results of the performance tests.
    """
    
    duration = data['duration']
    results = data['results']

    bandwidths = [result['bandwidth'] // 1000 for result in results]
    latencies = [result['seconds'] - duration for result in results]

    plt.plot(bandwidths, latencies)
    plt.title('Bandwidth vs Latency')

    plt.xlabel('Bandwidth (kbps)')
    
    plt.ylabel('Latency (s)')
    # plt.ylim(0, 10)
    
    plt.show()

if __name__ == '__main__':
    file_path = sys.argv[1]
    data = read_json_file(file_path)
    plot_results(data)