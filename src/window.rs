use pyo3::prelude::*;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder
};

#[pyclass]
#[pyo3(text_signature="(title: str, width: int, height: int)")]
pub struct Window {
    title: String,
    width: u32,
    height: u32,
}

#[pymethods]
impl Window {
    #[new]
    /// Returns a new instance of [Window]
    pub fn new(title: &str, width: u32, height: u32) -> PyResult<Self> {
        Ok(Self { title: title.to_owned(), width, height })
    }

    #[getter]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[getter]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Starts running the event loop
    /// 
    /// Note: This is a blocking call.
    /// Ideally, should be the last method to be called.
    pub fn run(&self) {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(self.title.clone());

        let windowed_ctx = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .expect("failed to create window");
        
        let windowed_ctx = unsafe { windowed_ctx.make_current().unwrap() };

        // TODO: create skia renderer

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::Resized(physical_size) => windowed_ctx.resize(physical_size),
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                    _ => () // TODO
                },
                _ => () // TODO
            }
        })
    }
}

pub fn add_window_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Window>()?;

    Ok(())
}