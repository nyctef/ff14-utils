import pandas as pd
from matplotlib import pyplot as plt
import sys

plt.rcParams['figure.autolayout'] = True

df = pd.read_csv(sys.argv[1])
df.plot(logy=True)
plt.show()
