extern crate cpython;

use self::cpython::{Python, PyDict, PyResult, NoArgs};
use LineTiming;
use std::collections::HashMap;

type ChartData = HashMap<u32, (Vec<f64>, Vec< Vec<f64> >)>;


pub fn show(data: Vec<LineTiming>){
    let chart_data: ChartData = format_data(data);
    let gil = Python::acquire_gil();
    chart(gil.python(), chart_data).unwrap();
}

fn chart(py: Python, data: ChartData) -> PyResult<()> {
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

fn format_data(line_timings: Vec<LineTiming>) -> ChartData{
    let mut chart_data: ChartData = HashMap::new();
    for line in line_timings{
        let LineTiming{ 
            line_number,
            top_durations,
            average_of_line,
        } = line;
        
        let top_durations: Vec<f64> = top_durations.iter().map(|t| t.1).collect();
        if !chart_data.contains_key(&line_number) {
            let mut top_rows: Vec<Vec<f64>> = Vec::new();
            for t in top_durations {
                top_rows.push( vec![t] );
            }
            let timing = (vec![average_of_line.1], top_rows);
            chart_data.insert(line_number, timing);
        }else{
            let mut cd = chart_data.get_mut(&line_number).unwrap();
            cd.0.push(average_of_line.1);
            for (i, top) in cd.1.iter_mut().enumerate() {
                top.push(top_durations[i])
            }
        }
    }
    chart_data
}
