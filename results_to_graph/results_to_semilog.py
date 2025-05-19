import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

rect_data = pd.read_csv("experiment_results.csv")

plt.figure(figsize=(8, 6))
plt.semilogx(rect_data['input_size'], rect_data['hull_size'], marker='o', linestyle='-', label='Rectangle')
plt.title('Semi-Log Plot Of Convex Hull Size (Rectangle)')
plt.xlabel('N (log scale)')
plt.ylabel('Hull size (log scale)')
plt.legend()
plt.tight_layout()
plt.savefig('rectangle_semilogx.png')
plt.show()
