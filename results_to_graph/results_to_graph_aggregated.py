import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

df = pd.read_csv('experiment_results.csv')
df['input_size'] = pd.to_numeric(df['input_size'], errors='coerce')
df['duration'] = pd.to_numeric(df['duration'], errors='coerce')

num_bins = 1000

bin_edges = np.linspace(df['input_size'].min(), df['input_size'].max(), num_bins + 1)
df['bin'] = pd.cut(df['input_size'], bins=bin_edges, include_lowest=True)

agg_df = df.groupby('bin').agg({
    'input_size': 'mean',
    'duration': 'mean'
}).reset_index()

plt.figure(figsize=(8, 6))
plt.plot(agg_df['input_size'], agg_df['duration'], marker='o', linestyle='-', markersize=3, label='Trend')
plt.xlabel('Input Size n')
plt.ylabel('Average Duration')
plt.title('Trend of Time Complexity Across Input Sizes')
plt.legend()

plt.savefig('graph.png', bbox_inches='tight')
plt.close()
