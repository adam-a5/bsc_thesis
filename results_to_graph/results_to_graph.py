import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

df = pd.read_csv('experiment_results.csv')
df['input_size'] = pd.to_numeric(df['input_size'], errors='coerce')
df['duration'] = pd.to_numeric(df['duration'], errors='coerce')

plt.figure(figsize=(8, 6))
plt.plot(df['input_size'], df['duration'], marker='o', linestyle='-', markersize=3, label='Trend')
plt.xlabel('Input Size n')
plt.ylabel('Average Duration')
plt.title('Trend of Time Complexity Across Input Sizes')
plt.legend()

plt.savefig('graph.png', bbox_inches='tight')
plt.close()
