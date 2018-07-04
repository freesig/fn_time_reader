import pygal
from collections import defaultdict

def show(data):
    total = 0
    line_chart = pygal.Line(width=1200, truncate_legend= -1)
    line_chart.title = 'Timings'
    averages = defaultdict(list)
    tops = []
    counts_dict = defaultdict(list)
    for (capture, counts) in data:
        total += 1
        for (line_number, timing) in capture.items():
            averages[line_number].append(timing[0])
            for i, top in enumerate(timing[1]):
                if i >= len(tops):
                    tops.append(defaultdict(list))
                tops[i][line_number].append(top)
        for (line_number, c) in counts.items():
            counts_dict[line_number].append(c)

    for line_number, timing in averages.items():
        line_chart.add(str(line_number) + " average", timing) 
    for top in tops:
        for line_number, timing in top.items():
            line_chart.add(str(line_number), timing) 
    for line_number, count in counts_dict.items():
        line_chart.add(str(line_number), count, secondary=True)
    
    line_chart.render_to_file('chart.svg')
    print("Total length: ", total)
