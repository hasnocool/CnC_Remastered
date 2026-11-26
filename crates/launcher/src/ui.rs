// Filename: crates/launcher/src/ui.rs
//! Immediate-mode graphical launcher and settings screens.

use renderer::{Color, UiFrame, Viewport};
use crate::{application::{application_info, ApplicationId}, config::LauncherConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiAction { None, Launch(ApplicationId), Quit, SettingsChanged }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen { Home, Settings }

pub struct LauncherUi {
    screen: Screen,
    selected: ApplicationId,
    pointer: (f32, f32),
    status: String,
}
impl Default for LauncherUi {
    fn default() -> Self {
        Self { screen: Screen::Home, selected: ApplicationId::TiberianDawn, pointer: (0.0,0.0), status: String::new() }
    }
}

impl LauncherUi {
    pub fn set_pointer(&mut self, x: f32, y: f32) { self.pointer = (x,y); }

    pub fn draw(&self, viewport: Viewport, config: &LauncherConfig) -> UiFrame {
        let mut ui = UiFrame::new();
        let w = viewport.width as f32;
        let h = viewport.height as f32;
        ui.rect(viewport,0.0,0.0,w,h,Color::DARK_MENU);
        ui.rect(viewport,0.0,0.0,w,8.0,Color::ACCENT);
        ui.text(viewport,48.0,36.0,4.0,Color::ACCENT,"COMMAND AND CONQUER");
        ui.text(viewport,50.0,68.0,1.8,Color::rgba(0.70,0.76,0.80,1.0),"REMastered RTS COLLECTION");
        ui.rect(viewport,48.0,105.0,(w-96.0).max(0.0),1.0,Color::rgba(0.30,0.36,0.39,1.0));

        match self.screen {
            Screen::Home => self.draw_home(viewport,&mut ui),
            Screen::Settings => self.draw_settings(viewport,config,&mut ui),
        }
        if !self.status.is_empty() {
            ui.rect(viewport,48.0,h-142.0,(w-96.0).max(0.0),1.0,Color::rgba(0.25,0.31,0.35,1.0));
            ui.text(viewport,52.0,h-125.0,1.5,Color::rgba(0.93,0.72,0.35,1.0),&self.status);
        }
        ui
    }

    fn draw_home(&self, viewport: Viewport, ui: &mut UiFrame) {
        let w=viewport.width as f32; let h=viewport.height as f32;
        ui.text(viewport,50.0,135.0,2.0,Color::rgba(0.88,0.91,0.91,1.0),"SELECT YOUR EXPERIENCE");
        let margin=48.0; let gap=18.0;
        let card_w=((w-margin*2.0-gap*2.0)/3.0).min(360.0).max(180.0);
        let total=card_w*3.0+gap*2.0;
        let start=((w-total)*0.5).max(margin);
        let card_y=190.0; let card_h=(h-360.0).clamp(150.0,230.0);
        let apps=[
            (ApplicationId::TiberianDawn,"TIBERIAN DAWN","1995  |  GDI VS NOD",Color::rgba(0.19,0.34,0.20,1.0)),
            (ApplicationId::RedAlert,"RED ALERT","1996  |  ALLIES VS SOVIETS",Color::rgba(0.39,0.20,0.18,1.0)),
            (ApplicationId::MapEditor,"MAP EDITOR","CREATE AND EDIT BATTLEFIELDS",Color::rgba(0.19,0.29,0.39,1.0)),
        ];
        for (index,(id,title,subtitle,accent)) in apps.into_iter().enumerate() {
            let x=start+index as f32*(card_w+gap);
            let selected=self.selected==id;
            let hovered=inside(self.pointer,(x,card_y,card_w,card_h));
            let background=if selected { Color::rgba(0.12,0.16,0.17,1.0) } else { Color::PANEL };
            ui.rect(viewport,x,card_y,card_w,card_h,background);
            ui.rect(viewport,x,card_y,card_w,7.0,accent);
            ui.outline(viewport,x,card_y,card_w,card_h,if selected {3.0}else{1.0},
                if selected {Color::ACCENT}else if hovered {Color::rgba(0.55,0.62,0.63,1.0)} else {Color::rgba(0.22,0.29,0.32,1.0)});
            ui.text(viewport,x+20.0,card_y+34.0,2.0,Color::rgba(0.95,0.94,0.86,1.0),title);
            ui.text(viewport,x+20.0,card_y+75.0,1.15,Color::rgba(0.66,0.73,0.73,1.0),subtitle);
            ui.rect(viewport,x+20.0,card_y+card_h-50.0,card_w-40.0,1.0,Color::rgba(0.29,0.36,0.37,1.0));
            ui.text(viewport,x+20.0,card_y+card_h-34.0,1.0,
                if selected {Color::ACCENT}else{Color::rgba(0.61,0.69,0.69,1.0)},
                if selected {"SELECTED"} else {"CHOOSE APPLICATION"});
        }
        let bottom=h-82.0;
        button(ui,viewport,48.0,bottom,164.0,46.0,"SETTINGS",inside(self.pointer,(48.0,bottom,164.0,46.0)),false);
        button(ui,viewport,230.0,bottom,112.0,46.0,"QUIT",inside(self.pointer,(230.0,bottom,112.0,46.0)),false);
        let launch_x=w-244.0;
        button(ui,viewport,launch_x,bottom,196.0,46.0,"CONTINUE",inside(self.pointer,(launch_x,bottom,196.0,46.0)),true);
        ui.text(viewport,50.0,h-24.0,1.0,Color::rgba(0.43,0.51,0.53,1.0),"SELECT AN APPLICATION  /  SETTINGS  /  CONTINUE");
    }

    fn draw_settings(&self, viewport: Viewport, config: &LauncherConfig, ui: &mut UiFrame) {
        let w=viewport.width as f32; let h=viewport.height as f32;
        ui.text(viewport,50.0,138.0,2.0,Color::rgba(0.91,0.93,0.91,1.0),"APPLICATION SETTINGS");
        let rows=[
            ("FULLSCREEN",if config.display.fullscreen {"ON"}else{"OFF"}),
            ("VERTICAL SYNC",if config.display.vsync {"ON"}else{"OFF"}),
            ("LOW POWER GPU",if config.performance.low_power_adapter {"ON"}else{"OFF"}),
            ("NETWORKING",if config.network.enabled {"ON"}else{"OFF"}),
            ("MASTER AUDIO",if config.audio.master_volume>0.0 {"ON"}else{"MUTED"}),
            ("EDGE SCROLLING",if config.input.edge_scrolling {"ON"}else{"OFF"}),
            ("INVERT MOUSE Y",if config.input.invert_mouse_y {"ON"}else{"OFF"}),
        ];
        let x=64.0; let y0=174.0; let row_h=48.0;
        for (index,(label,value)) in rows.iter().enumerate() {
            let y=y0+index as f32*row_h;
            ui.rect(viewport,x,y,(w-128.0).max(0.0),48.0,Color::PANEL);
            ui.text(viewport,x+18.0,y+17.0,1.5,Color::rgba(0.86,0.89,0.87,1.0),label);
            let bx=w-210.0;
            button(ui,viewport,bx,y+6.0,124.0,36.0,value,inside(self.pointer,(bx,y+6.0,124.0,36.0)),true);
        }
        let resolution_y=y0+rows.len() as f32*row_h+2.0;
        ui.text(viewport,x+4.0,resolution_y+12.0,1.4,Color::rgba(0.82,0.87,0.86,1.0),"RESOLUTION");
        ui.text(viewport,x+230.0,resolution_y+12.0,1.4,Color::ACCENT,&format!("{} X {}",config.display.width,config.display.height));
        ui.text(viewport,x+4.0,resolution_y+48.0,1.2,Color::rgba(0.55,0.64,0.65,1.0),"FRAME LIMIT");
        let fps=format!("{} FPS",config.performance.frame_limit);
        ui.text(viewport,w-210.0,resolution_y+48.0,1.4,Color::ACCENT,&fps);
        button(ui,viewport,w-275.0,resolution_y+38.0,34.0,28.0,"-",inside(self.pointer,(w-275.0,resolution_y+38.0,34.0,28.0)),false);
        button(ui,viewport,w-72.0,resolution_y+38.0,34.0,28.0,"+",inside(self.pointer,(w-72.0,resolution_y+38.0,34.0,28.0)),false);
        let bottom=h-82.0;
        button(ui,viewport,48.0,bottom,164.0,46.0,"BACK",inside(self.pointer,(48.0,bottom,164.0,46.0)),false);
        ui.text(viewport,245.0,bottom+17.0,1.0,Color::rgba(0.52,0.60,0.61,1.0),"SETTINGS SAVE AUTOMATICALLY");
    }

    pub fn click(&mut self, viewport: Viewport, config: &mut LauncherConfig, x: f32, y: f32) -> UiAction {
        let w=viewport.width as f32; let h=viewport.height as f32;
        match self.screen {
            Screen::Home => {
                let margin=48.0; let gap=18.0;
                let card_w=((w-margin*2.0-gap*2.0)/3.0).min(360.0).max(180.0);
                let total=card_w*3.0+gap*2.0;
                let start=((w-total)*0.5).max(margin);
                let card_y=190.0; let card_h=(h-360.0).clamp(150.0,230.0);
                let ids=[ApplicationId::TiberianDawn,ApplicationId::RedAlert,ApplicationId::MapEditor];
                for (index,id) in ids.into_iter().enumerate() {
                    let rect=(start+index as f32*(card_w+gap),card_y,card_w,card_h);
                    if inside((x,y),rect) { self.selected=id; self.status.clear(); return UiAction::None; }
                }
                let bottom=h-82.0;
                if inside((x,y),(48.0,bottom,164.0,46.0)) { self.screen=Screen::Settings; self.status.clear(); }
                else if inside((x,y),(230.0,bottom,112.0,46.0)) { return UiAction::Quit; }
                else if inside((x,y),(w-244.0,bottom,196.0,46.0)) {
                    let name=application_info(self.selected).name;
                    self.status=format!("{name} selected — game/editor runtime integration is the next stage");
                    return UiAction::Launch(self.selected);
                }
            }
            Screen::Settings => {
                let y0=174.0; let row_h=48.0; let bx=w-210.0;
                for index in 0..7 {
                    let y=y0+index as f32*row_h+6.0;
                    if inside((x,y),(bx,y,124.0,36.0)) {
                        match index {
                            0=>config.display.fullscreen=!config.display.fullscreen,
                            1=>config.display.vsync=!config.display.vsync,
                            2=>config.performance.low_power_adapter=!config.performance.low_power_adapter,
                            3=>config.network.enabled=!config.network.enabled,
                            4=>config.audio.master_volume=if config.audio.master_volume>0.0 {0.0}else{1.0},
                            5=>config.input.edge_scrolling=!config.input.edge_scrolling,
                            6=>config.input.invert_mouse_y=!config.input.invert_mouse_y,
                            _=>{}
                        }
                        return UiAction::SettingsChanged;
                    }
                }
                let resolution_y=y0+7.0*row_h+2.0;
                if inside((x,y),(w-275.0,resolution_y+38.0,34.0,28.0)) {
                    config.performance.frame_limit=config.performance.frame_limit.saturating_sub(15).max(30);
                    return UiAction::SettingsChanged;
                }
                if inside((x,y),(w-72.0,resolution_y+38.0,34.0,28.0)) {
                    config.performance.frame_limit=config.performance.frame_limit.saturating_add(15).min(240);
                    return UiAction::SettingsChanged;
                }
                let bottom=h-82.0;
                if inside((x,y),(48.0,bottom,164.0,46.0)) { self.screen=Screen::Home; }
            }
        }
        UiAction::None
    }
}

fn button(ui:&mut UiFrame,viewport:Viewport,x:f32,y:f32,w:f32,h:f32,label:&str,hovered:bool,primary:bool) {
    let fill=if primary { if hovered {Color::rgba(0.88,0.68,0.31,1.0)}else{Color::rgba(0.63,0.45,0.17,1.0)} }
        else if hovered {Color::rgba(0.20,0.27,0.29,1.0)} else {Color::rgba(0.11,0.15,0.17,1.0)};
    ui.rect(viewport,x,y,w,h,fill);
    ui.outline(viewport,x,y,w,h,1.0,if primary {Color::ACCENT}else{Color::rgba(0.34,0.42,0.43,1.0)});
    let text_color=if primary {Color::rgba(0.05,0.07,0.07,1.0)}else{Color::rgba(0.90,0.92,0.89,1.0)};
    ui.text(viewport,x+16.0,y+(h-7.0*1.35)*0.5,1.35,text_color,label);
}
fn inside(point:(f32,f32),rect:(f32,f32,f32,f32))->bool {
    point.0>=rect.0 && point.1>=rect.1 && point.0<=rect.0+rect.2 && point.1<=rect.1+rect.3
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn menu_defaults_to_tiberian_dawn_and_settings_toggle_is_persistable() {
        let mut ui=LauncherUi::default();
        let mut config=LauncherConfig::default();
        let viewport=Viewport::new(1280,720);
        let action=ui.click(viewport,&mut config,48.0+10.0,720.0-82.0+10.0);
        assert_eq!(action,UiAction::None);
        assert_eq!(ui.screen,Screen::Settings);
        let action=ui.click(viewport,&mut config,1280.0-210.0+10.0,196.0+10.0);
        assert_eq!(action,UiAction::SettingsChanged);
        assert!(config.display.fullscreen);
    }
    #[test] fn selecting_red_alert_returns_it_from_continue_action() {
        let mut ui=LauncherUi::default(); let mut config=LauncherConfig::default();
        let viewport=Viewport::new(1280,720);
        let w=1280.0; let margin=48.0; let gap=18.0; let cw=((w-margin*2.0-gap*2.0)/3.0).min(360.0).max(180.0);
        let total=cw*3.0+gap*2.0; let start=((w-total)*0.5).max(margin);
        ui.click(viewport,&mut config,start+cw+gap+20.0,220.0);
        let action=ui.click(viewport,&mut config,w-244.0+10.0,720.0-82.0+10.0);
        assert_eq!(action,UiAction::Launch(ApplicationId::RedAlert));
    }
}
