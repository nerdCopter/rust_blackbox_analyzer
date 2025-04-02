use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

/// Structure to hold parsed data from the blackbox log
#[derive(Debug)]
struct LogData {
    loop_iteration: u32,
    time: u32,
    p_roll: f32,
    i_roll: f32,
    d_roll: f32,
    ff_roll: f32,
    p_pitch: f32,
    i_pitch: f32,
    d_pitch: f32,
    ff_pitch: f32,
    p_yaw: f32,
    i_yaw: f32,
    d_yaw: f32,
    ff_yaw: f32,
}

#[derive(Parser)]
#[command(name = "Blackbox Analyzer")]
#[command(version = "1.0")]
#[command(about = "Analyze Blackbox logs and output PID values to CSV and graphics", long_about = None)]
struct Args {
    /// Path to the blackbox log file
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Extract base name of the input file
    let input_base_name = args.input.file_stem().unwrap().to_str().unwrap();

    // Generate output filenames
    let csv_file_name = format!("{}.csv", input_base_name);
    let graphics_file_name = format!("{}.png", input_base_name);

    // Read and parse the log data
    let log_data = read_blackbox_log(&args.input)?;

    // Write PID values to a CSV file
    write_to_csv(&log_data, &csv_file_name)?;

    println!("CSV file generated: {}", csv_file_name);

    // Generate a plot of roll, pitch, and yaw over time
    generate_plot(&log_data, &graphics_file_name)?;

    println!("Graphics file generated: {}", graphics_file_name);

    Ok(())
}

/// Function to read and parse the blackbox log file
fn read_blackbox_log<P: AsRef<Path>>(file_path: P) -> Result<Vec<LogData>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut log_data: Vec<LogData> = Vec::new();

    loop {
        // Read fields from binary data
        let loop_iteration = match reader.read_u32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let time = match reader.read_u32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let p_roll = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let i_roll = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let d_roll = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let ff_roll = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let p_pitch = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let i_pitch = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let d_pitch = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let ff_pitch = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let p_yaw = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let i_yaw = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let d_yaw = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };
        let ff_yaw = match reader.read_f32::<LittleEndian>() {
            Ok(value) => value,
            Err(_) => break, // End of file or error
        };

        log_data.push(LogData {
            loop_iteration,
            time,
            p_roll,
            i_roll,
            d_roll,
            ff_roll,
            p_pitch,
            i_pitch,
            d_pitch,
            ff_pitch,
            p_yaw,
            i_yaw,
            d_yaw,
            ff_yaw,
        });
    }

    Ok(log_data)
}

/// Function to write PID values to a CSV file
fn write_to_csv(data: &[LogData], output_file: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file)?;

    writeln!(
        &mut file,
        "LoopIteration,Time,P_Roll,I_Roll,D_Roll,FF_Roll,P_Roll_NoFF,I_Roll_NoFF,D_Roll_NoFF,\
         P_Pitch,I_Pitch,D_Pitch,FF_Pitch,P_Pitch_NoFF,I_Pitch_NoFF,D_Pitch_NoFF,\
         P_Yaw,I_Yaw,D_Yaw,FF_Yaw,P_Yaw_NoFF,I_Yaw_NoFF,D_Yaw_NoFF"
    )?;

    for entry in data {
        writeln!(
            &mut file,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            entry.loop_iteration,
            entry.time,
            // Roll values
            entry.p_roll * 0.004,
            entry.i_roll * 0.004,
            entry.d_roll * 0.004,
            entry.ff_roll * 0.004,
            // Roll without FF
            (entry.p_roll - entry.ff_roll) * 0.004,
            (entry.i_roll - entry.ff_roll) * 0.004,
            (entry.d_roll - entry.ff_roll) * 0.004,
            // Pitch values
            entry.p_pitch * 0.004,
            entry.i_pitch * 0.004,
            entry.d_pitch * 0.004,
            entry.ff_pitch * 0.004,
            // Pitch without FF
            (entry.p_pitch - entry.ff_pitch) * 0.004,
            (entry.i_pitch - entry.ff_pitch) * 0.004,
            (entry.d_pitch - entry.ff_pitch) * 0.004,
            // Yaw values
            entry.p_yaw * 0.004,
            entry.i_yaw * 0.004,
            entry.d_yaw * 0.004,
            entry.ff_yaw * 0.004,
            // Yaw without FF
            (entry.p_yaw - entry.ff_yaw) * 0.004,
            (entry.i_yaw - entry.ff_yaw) * 0.004,
            (entry.d_yaw - entry.ff_yaw) * 0.004
        )?;
    }
    Ok(())
}

/// Function to generate a plot of roll, pitch, and yaw over time (optional)
fn generate_plot(log_data: &[LogData], output_file: &str) -> Result<()> {
    use plotters::prelude::*;
    use plotters::style::{BLACK, BLUE, GREEN, RED, WHITE};
    let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Roll, Pitch, Yaw over Time", ("sans-serif", 24).into_font())
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_2d(
            0u32..log_data.last().unwrap().time.max(1),
            -180f32..180f32,
        )?;

    chart.configure_mesh()
        .x_desc("Time (ms)")
        .y_desc("Angle (degrees)")
        .draw()?;

    // Scatter plot for Roll
    chart.draw_series(log_data.iter().map(|entry| {
        Circle::new((entry.time, entry.p_roll), 2, RED.filled())
    }))?
    .label("Roll")
    .legend(|(x, y)| Circle::new((x, y), 4, RED.filled()));

    // Scatter plot for Pitch
    chart.draw_series(log_data.iter().map(|entry| {
        Circle::new((entry.time, entry.p_pitch), 2, GREEN.filled())
    }))?
    .label("Pitch")
    .legend(|(x, y)| Circle::new((x, y), 4, GREEN.filled()));

    // Scatter plot for Yaw
    chart.draw_series(log_data.iter().map(|entry| {
        Circle::new((entry.time, entry.p_yaw), 2, BLUE.filled())
    }))?
    .label("Yaw")
    .legend(|(x, y)| Circle::new((x, y), 4, BLUE.filled()));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;
    Ok(())
}
