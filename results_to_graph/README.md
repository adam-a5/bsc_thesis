# Graphing Component

Originally, the file `results_to_graph.py` was used. However, this does not work well with a large number of points. It consumes a lot of memory, takes a long time to compute, and the labels are all layed on top of each other.

A solution to this problem is to aggregate the data into bins of 1,000. This is implemented in `results_to_graph_aggregated.py`.