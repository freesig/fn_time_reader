extern crate cpython;

use self::cpython::{Python, PyDict, PyResult, NoArgs};
use StreamData;
use ft::Counter;
use ft::LineTiming;
use std::collections::HashMap;

type ChartCapture = HashMap<u32, (f64, Vec<f64>)>;
type ChartCount = HashMap<u32, usize>;

type ChartData = (ChartCapture, ChartCount);

struct ChartDataTyped {
    pub capture: ChartCapture,
    pub counts: ChartCount,
}

pub fn show(data: Vec<StreamData>){
    let chart_data = format_data(data);
    let gil = Python::acquire_gil();
    chart(gil.python(), chart_data).unwrap();
}

fn chart(py: Python, data: Vec<ChartData>) -> PyResult<()> {
    let os = py.import("os")?;
    let cwd = os.call(py, "getcwd", NoArgs, None)?;
    let cwd = format!("{}{}", cwd, "/assets");
    
    let locals = PyDict::new(py);
    locals.set_item(py, "sys", py.import("sys")?)?;
    let add_assets_dir = format!("sys.path.append('{}')", cwd);
    py.eval(&add_assets_dir[..], None, Some(&locals))?;
    
    let chart = py.import("chart")?;
    chart.call(py, "show", (data,), None)?; 
    
    Ok(())
}

fn format_data(stream_data: Vec<StreamData>) -> Vec<ChartData>{
    let mut all_chart_data = Vec::<ChartData>::with_capacity(stream_data.len()); 

    for StreamData{ capture, count: counts } in stream_data{
        let mut chart_data = ChartDataTyped{ capture: HashMap::new(), counts: HashMap::new() };
        for line in capture {
            let LineTiming{ 
                line_number,
                top_durations,
                average_of_line,
            } = line;

            let top_durations: Vec<f64> = top_durations.iter().map(|t| t.1).collect();
            let timing = (average_of_line.1, top_durations);
            chart_data.capture.insert(line_number, timing);
        }

        for count in counts {
            let Counter { line, n } = count;
            chart_data.counts.insert(line, n);
        }


        // Convert chart data typed to simple tuple for python
        all_chart_data.push((chart_data.capture, chart_data.counts));
    }
    all_chart_data
}
