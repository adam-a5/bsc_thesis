import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('experiment_results.csv')
input_sizes = df['input_size'].values
durations = df['duration'].values

plt.figure(figsize=(8, 6))
plt.scatter(input_sizes, durations, label='Data', color='blue')
plt.xlabel('Input Size n')
plt.ylabel('Duration')
plt.title('Time Complexity Graph')

plt.savefig('graph.png', bbox_inches='tight')
plt.close()