import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

df = pd.read_csv('experiment_results.csv')
df['input_size'] = pd.to_numeric(df['input_size'], errors='coerce')
df['hull_size'] = pd.to_numeric(df['hull_size'], errors='coerce')

plt.figure(figsize=(8, 6))
plt.plot(df['input_size'], df['hull_size'], marker='o', linestyle='-', markersize=3, label='Trend')
plt.xlabel('Input Size n')
plt.ylabel('Average Hull Size')
plt.title('Growth of Convex Hull Size')
plt.legend()

plt.savefig('graph.png', bbox_inches='tight')
plt.close()
