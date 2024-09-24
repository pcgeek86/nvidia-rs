use crossterm::event::Event;
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};

use ratatui::{
    crossterm::event::{self,KeyCode,KeyEventKind}, layout::{Constraint, Layout}, style::{Style, Stylize}, symbols::Marker, widgets::{block::Title, Axis, Block, Chart, Dataset, Widget}, DefaultTerminal, Frame
};

mod tests;

fn main() {
    // get_nvidia_gpu_info();
    run_tui();
}

fn run_tui() {
    let mut terminal = ratatui::init();
    terminal.clear().expect("Failed to clear terminal");

    let mut app = NvidiaApp::default();
    let app_result = app.run_app(&mut terminal);

    ratatui::restore();
}

#[derive(Debug, Default)]
struct NvidiaApp {
    gpu_clock: [u32;30],
    gpu_device: String,
    exit: bool,
}

impl NvidiaApp {
    pub fn run_app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let nvml = Nvml::init().expect("Failed to initialize NVML");
        let gpu_device = nvml.device_by_index(0).expect("Failed to get device index 0");
        self.gpu_device = gpu_device.name().expect("Failed to get GPU part number");
        

        while !self.exit {
            self.update_state()?;
            let _ = terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // This function is used to update the main application state
    fn update_state(&mut self) -> std::io::Result<()> {
        let nvml = Nvml::init().expect("Failed to initialize NVML");
        let gpu_device = nvml.device_by_index(0).expect("Failed to get device index 0");
        let current_clock = gpu_device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics).unwrap();
        self.gpu_clock.rotate_left(1);
        self.gpu_clock[29] = current_clock;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) { 
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if !event::poll(std::time::Duration::from_millis(150)).unwrap() {
            return Ok(()); // Don't try to read any events if there aren't any available
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {  }
        }
        Ok(())
    }
    
    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.exit();
            },
            _ => { }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &NvidiaApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [parent01, parent02,] = Layout::horizontal(Constraint::from_percentages([50,50])).areas(area);

        let [top01, bottom01] = Layout::vertical(Constraint::from_percentages([50,50])).areas(parent01);
        let [top02, bottom02] = Layout::vertical(Constraint::from_percentages([50,50])).areas(parent02);

        
        let title = Title::from(format!("NVIDIA GPU Monitor {0}", self.gpu_device));
        let block01 = Block::bordered()
            .border_style(Style::new()
                .fg(ratatui::style::Color::Rgb(117, 255, 0))
            )
            .title(title.alignment(ratatui::layout::Alignment::Center));
        
        let block02 = Block::bordered()
            .style(Style::new().cyan());
        block02.render(top02, buf);
        

        let gpu_clock_data: [(f64, f64); 30] = self.gpu_clock.iter().zip(-29..=0).map(|i| (i.1 as f64, *i.0 as f64)).collect::<Vec<(f64,f64)>>().try_into().unwrap();
        let chart_gpu_clock_data = Dataset::default()
            .name("GPU Clock")
            .marker(Marker::Dot)
            .graph_type(ratatui::widgets::GraphType::Line)
            // .data(&[(0.0, 480.0), (-1.0, 420.0), (-2.0, 825.0), (-3.0, 600.0), (-4.0, 735.0,), (-5.0, 855.0), (-6.0, 600.0), (-7.0, 570.0), (-8.0, 1825.0), (-9.0, 1925.0)]);
            .data(&gpu_clock_data);
        let chart_gpu_clock_x_axis = Axis::default().title("Time").bounds([-30.0, 0.0]);
        let chart_gpu_clock_y_axis = Axis::default().title("GPU Clock Speed").bounds([0.0,2000.0]);
        let chart_gpu_clock = Chart::new(vec![chart_gpu_clock_data])
            .block(block01)
            .x_axis(chart_gpu_clock_x_axis)
            .y_axis(chart_gpu_clock_y_axis)
            .style(Style::new().fg(ratatui::style::Color::Rgb(48,226,173)));

        chart_gpu_clock.render(top01, buf);

    }
}

// Legacy function, not currently being called anywhere.
// Just used this to test retrieving certain values from the NVIDIA GPU.
#[allow(dead_code)]
fn get_nvidia_gpu_info() {
    let nvml = nvml_wrapper::Nvml::init().expect("Failed to initialize NVML");
    println!("You have {0} NVIDIA GPUs", nvml.device_count().expect("Failed to list NVIDIA GPUs"));

    println!("NVIDIA Driver version: {0}", nvml.sys_driver_version().expect("Failed to retrieve NVIDIA driver version"));
    println!("NVIDIA CUDA version: {0}", nvml.sys_cuda_driver_version().expect("Failed to retrieve NVIDIA CUDA version"));

    for gpu_index in 0..nvml.device_count().unwrap() {        
        let gpu = nvml.device_by_index(gpu_index).expect("Failed to retrieve GPU with that index");
        println!("{0}: GPU Architecture: {1}", gpu_index, gpu.architecture().expect("Failed to retrieve GPU architecture"));
        println!("{0}: GPU Brand: {1:?}", gpu_index, gpu.brand().expect("Failed to get GPU brand"));
        println!("{0}: 🌡️  GPU Temperature: {1:?} °C ", gpu_index, gpu.temperature(TemperatureSensor::Gpu).expect("Failed to retrieve GPU temperature"));
        println!("{0}: GPU Driver Model: {1:?}", gpu_index, gpu.driver_model().expect("Failed to retrieve GPU driver model").current);
        println!("{0}: GPU Power Consumption: {1} watts", gpu_index, (gpu.power_usage().expect("Failed to get GPU power consumption")/1000) as f32);
        
    }
}