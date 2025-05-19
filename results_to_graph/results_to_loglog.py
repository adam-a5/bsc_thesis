import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

disk_data = pd.read_csv("experiment_results.csv")

plt.figure(figsize=(8, 6))
plt.loglog(disk_data['input_size'], disk_data['hull_size'], marker='o', linestyle='-', label='Disk')
plt.title('Log-Log Plot of Expected Convex Hull Size (Disk)')
plt.xlabel('N (log scale)')
plt.ylabel('Hull size (log scale)')
plt.legend()
plt.tight_layout()
plt.savefig('disk_loglog.png')
plt.show()
