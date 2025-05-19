import pandas as pd
import numpy as np
from sklearn.linear_model import LinearRegression

df = pd.read_csv("experiment_results.csv")
n = np.log(df['input_size'].values)
hull_size = df['hull_size'].values

model = LinearRegression().fit(n.reshape(-1,1), hull_size)
print(f"R2: {model.score(n.reshape(-1,1), hull_size):.4f}")