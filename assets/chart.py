import pygal

def show(data):
    line_chart = pygal.Line()
    line_chart.title = 'Timings'
    for line_number, timing in data.items():
        line_chart.add(str(line_number) + " average", timing[0]) 
        for top in timing[1]:
            line_chart.add(str(line_number), top) 
    line_chart.render_to_file('chart.svg')
