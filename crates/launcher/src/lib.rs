// Filename: crates/launcher/src/lib.rs
//! Unified application launcher and lifecycle coordinator.

pub mod application;
pub mod config;
pub mod ui;

use std::{sync::{Arc, mpsc::{self, Receiver, TryRecvError}}, thread, time::{Duration, Instant}};
use application::{ApplicationId, ApplicationInfo};
use config::{LauncherConfig, SettingsStore};
use renderer::{Color, RendererError, Viewport, WgpuRenderer};
use tracing::{error, info};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowId},
};

#[derive(Debug)]
pub enum LauncherError { Config(String), Window(String), Renderer(RendererError), EventLoop(String) }
impl std::fmt::Display for LauncherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(e) => write!(f, "configuration error: {e}"),
            Self::Window(e) => write!(f, "window error: {e}"),
            Self::Renderer(e) => write!(f, "{e}"),
            Self::EventLoop(e) => write!(f, "event loop error: {e}"),
        }
    }
}
impl std::error::Error for LauncherError {}
impl From<RendererError> for LauncherError { fn from(value: RendererError) -> Self { Self::Renderer(value) } }

pub fn available_applications() -> [ApplicationInfo; 3] {
    [
        application::application_info(ApplicationId::TiberianDawn),
        application::application_info(ApplicationId::RedAlert),
        application::application_info(ApplicationId::MapEditor),
    ]
}


/// Start the graphical launcher. The renderer is initialized on a worker so
/// GPU adapter discovery never blocks the winit event loop.
pub fn launch_menu() -> Result<(), LauncherError> {
    let store = SettingsStore::discover().map_err(|e| LauncherError::Config(e.to_string()))?;
    let config = store.load().map_err(|e| LauncherError::Config(e.to_string()))?;
    let event_loop = EventLoop::new().map_err(|e| LauncherError::EventLoop(e.to_string()))?;
    let mut runtime = MenuRuntime::new(config, store);
    event_loop.run_app(&mut runtime).map_err(|e| LauncherError::EventLoop(e.to_string()))?;
    if let Some(error) = runtime.startup_error { return Err(LauncherError::Window(error)); }
    Ok(())
}

struct MenuRuntime {
    config: LauncherConfig,
    store: SettingsStore,
    ui: ui::LauncherUi,
    window: Option<Arc<Window>>,
    renderer: Option<WgpuRenderer>,
    renderer_rx: Option<Receiver<Result<WgpuRenderer, RendererError>>>,
    pointer: (f32, f32),
    startup_error: Option<String>,
}
impl MenuRuntime {
    fn new(config: LauncherConfig, store: SettingsStore) -> Self {
        Self { config, store, ui: ui::LauncherUi::default(), window: None, renderer: None,
            renderer_rx: None, pointer: (0.0,0.0), startup_error: None }
    }
    fn persist(&mut self) {
        if let Err(error) = self.store.save(&self.config) {
            self.startup_error = Some(error.to_string());
            error!("settings persistence failed: {error}");
        }
    }
    fn draw(&self) {
        let (Some(renderer),Some(window))=(&self.renderer,&self.window) else { return };
        let viewport=renderer.viewport();
        let frame=self.ui.draw(viewport,&self.config);
        let _=window;
        // A transient surface error is handled by the next redraw; fatal errors
        // are captured by the window event path.
        let _=frame;
    }
}

impl ApplicationHandler for MenuRuntime {
    fn resumed(&mut self,event_loop:&ActiveEventLoop) {
        if self.window.is_some(){return;}
        let attributes=Window::default_attributes()
            .with_title("CnC Remastered — Command Center")
            .with_inner_size(LogicalSize::new(self.config.display.width,self.config.display.height));
        let window=match event_loop.create_window(attributes){
            Ok(window)=>Arc::new(window),
            Err(error)=>{self.startup_error=Some(error.to_string());event_loop.exit();return;}
        };
        let renderer_window=Arc::clone(&window);
        let vsync=self.config.display.vsync;
        let low_power=self.config.performance.low_power_adapter;
        let (sender,receiver)=mpsc::sync_channel(1);
        if let Err(error)=thread::Builder::new().name("cnc-launcher-gpu-init".to_owned()).spawn(move||{
            let result=pollster::block_on(WgpuRenderer::new(renderer_window,vsync,low_power));
            let _=sender.send(result);
        }){
            self.startup_error=Some(error.to_string());event_loop.exit();return;
        }
        self.window=Some(window);self.renderer_rx=Some(receiver);
    }
    fn window_event(&mut self,event_loop:&ActiveEventLoop,window_id:WindowId,event:WindowEvent){
        let Some(window)=self.window.as_ref() else{return};
        if window.id()!=window_id{return;}
        match event {
            WindowEvent::CloseRequested=>event_loop.exit(),
            WindowEvent::Resized(size)=>{
                if let Some(renderer)=self.renderer.as_mut(){renderer.resize(Viewport::new(size.width,size.height));}
                window.request_redraw();
            }
            WindowEvent::CursorMoved{position,..}=>{
                self.pointer=(position.x as f32,position.y as f32);
                self.ui.set_pointer(self.pointer.0,self.pointer.1);
                window.request_redraw();
            }
            WindowEvent::MouseInput{state:winit::event::ElementState::Pressed,button:winit::event::MouseButton::Left,..}=>{
                if let Some(renderer)=self.renderer.as_ref(){
                    let action=self.ui.click(renderer.viewport(),&mut self.config,self.pointer.0,self.pointer.1);
                    match action {
                        ui::UiAction::Quit=>event_loop.exit(),
                        ui::UiAction::SettingsChanged=>self.persist(),
                        ui::UiAction::Launch(application)=>{
                            info!(application=%application,"application selected from launcher");
                        }
                        ui::UiAction::None=>{}
                    }
                }
                window.request_redraw();
            }
            WindowEvent::KeyboardInput{event,..} if event.state==winit::event::ElementState::Pressed
                && event.logical_key==winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape)=>{
                    event_loop.exit();
                }
            WindowEvent::RedrawRequested=>{
                if let Some(renderer)=self.renderer.as_mut(){
                    let frame=self.ui.draw(renderer.viewport(),&self.config);
                    if let Err(error)=renderer.render(Color::DARK_MENU,None,&frame){
                        self.startup_error=Some(error.to_string());
                        error!("launcher frame failed: {error}");
                        event_loop.exit();
                    }
                }
            }
            _=>{}
        }
    }
    fn about_to_wait(&mut self,event_loop:&ActiveEventLoop){
        if let Some(receiver)=self.renderer_rx.as_ref(){
            match receiver.try_recv(){
                Ok(Ok(renderer))=>{self.renderer=Some(renderer);self.renderer_rx=None;info!("launcher renderer initialized");}
                Ok(Err(error))=>{self.startup_error=Some(error.to_string());self.renderer_rx=None;error!("launcher renderer init failed: {error}");event_loop.exit();return;}
                Err(TryRecvError::Disconnected)=>{self.startup_error=Some("renderer worker stopped unexpectedly".to_owned());self.renderer_rx=None;event_loop.exit();return;}
                Err(TryRecvError::Empty)=>{}
            }
        }
        if let Some(window)=self.window.as_ref(){if self.renderer.is_some(){window.request_redraw();}}
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now()+Duration::from_millis(16)));
    }
    fn exiting(&mut self,_event_loop:&ActiveEventLoop){
        self.renderer=None;self.window=None;info!("graphical launcher shut down");
    }
}

pub fn launch(application: ApplicationId) -> Result<(), LauncherError> {
    let store = SettingsStore::discover().map_err(|e| LauncherError::Config(e.to_string()))?;
    let mut config = store.load().map_err(|e| LauncherError::Config(e.to_string()))?;
    config.last_application = application;
    store.save(&config).map_err(|e| LauncherError::Config(e.to_string()))?;
    let event_loop = EventLoop::new().map_err(|e| LauncherError::EventLoop(e.to_string()))?;
    let mut runtime = Runtime::new(application, config);
    event_loop.run_app(&mut runtime).map_err(|e| LauncherError::EventLoop(e.to_string()))?;
    if let Some(error) = runtime.startup_error { return Err(LauncherError::Window(error)); }
    Ok(())
}

struct Runtime {
    application: ApplicationId,
    config: LauncherConfig,
    window: Option<Arc<Window>>,
    renderer: Option<WgpuRenderer>,
    renderer_rx: Option<Receiver<Result<WgpuRenderer, RendererError>>>,
    startup_error: Option<String>,
}
impl Runtime {
    fn new(application: ApplicationId, config: LauncherConfig) -> Self {
        Self { application, config, window: None, renderer: None, renderer_rx: None, startup_error: None }
    }
    fn application_color(&self) -> Color {
        match self.application {
            ApplicationId::TiberianDawn => Color::rgba(0.10, 0.20, 0.12, 1.0),
            ApplicationId::RedAlert => Color::rgba(0.28, 0.10, 0.10, 1.0),
            ApplicationId::MapEditor => Color::rgba(0.10, 0.15, 0.25, 1.0),
        }
    }
}

impl ApplicationHandler for Runtime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() { return; }
        let info = application::application_info(self.application);
        let mut attributes = Window::default_attributes()
            .with_title(format!("CnC Remastered — {}", info.name))
            .with_inner_size(LogicalSize::new(self.config.display.width, self.config.display.height));
        if self.config.display.fullscreen {
            attributes = attributes.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                self.startup_error = Some(error.to_string());
                error!("window creation failed: {error}");
                event_loop.exit();
                return;
            }
        };

        // GPU initialization runs on a worker; event-loop polling never blocks.
        let renderer_window = Arc::clone(&window);
        let vsync = self.config.display.vsync;
        let low_power = self.config.performance.low_power_adapter;
        let (sender, receiver) = mpsc::sync_channel(1);
        if let Err(error) = thread::Builder::new().name("cnc-renderer-init".to_owned()).spawn(move || {
            let result = pollster::block_on(WgpuRenderer::new(renderer_window, vsync, low_power));
            let _ = sender.send(result);
        }) {
            self.startup_error = Some(error.to_string());
            event_loop.exit();
            return;
        }
        self.window = Some(window);
        self.renderer_rx = Some(receiver);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.as_ref() else { return };
        if window.id() != window_id { return; }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() { renderer.resize(Viewport::new(size.width, size.height)); }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    if let Err(error) = renderer.render_clear(self.application_color()) {
                        error!("frame presentation failed: {error}");
                        self.startup_error = Some(error.to_string());
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(receiver) = self.renderer_rx.as_ref() {
            match receiver.try_recv() {
                Ok(Ok(renderer)) => {
                    self.renderer = Some(renderer);
                    self.renderer_rx = None;
                    info!(application = %self.application, "renderer initialized");
                }
                Ok(Err(error)) => {
                    self.startup_error = Some(error.to_string());
                    error!("renderer initialization failed: {error}");
                    self.renderer_rx = None;
                    event_loop.exit();
                    return;
                }
                Err(TryRecvError::Disconnected) => {
                    self.startup_error = Some("renderer initialization worker stopped unexpectedly".to_owned());
                    self.renderer_rx = None;
                    event_loop.exit();
                    return;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if let Some(window) = self.window.as_ref() {
            if self.renderer.is_some() { window.request_redraw(); }
        }
        let fps = self.config.performance.frame_limit.clamp(1, 1000) as f64;
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_secs_f64(1.0 / fps)));
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer = None;
        self.window = None;
        info!("application runtime shut down");
    }
}
