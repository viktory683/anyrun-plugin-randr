use std::fmt;

pub mod dummy;
pub mod hyprland;

#[derive(PartialEq)]
pub struct Monitor {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub refresh_rate: f32,
    pub name: String,
    pub id: u64,
}

pub enum Configure<'a> {
    Mirror(&'a Monitor),
    LeftOf(&'a Monitor),
    RightOf(&'a Monitor),
    Below(&'a Monitor),
    Above(&'a Monitor),
    Zero,
}

impl<'a> Configure<'a> {
    pub fn from_id(id: u32, mon: &'a Monitor) -> Self {
        match id {
            0 => Configure::Mirror(mon),
            1 => Configure::LeftOf(mon),
            2 => Configure::RightOf(mon),
            3 => Configure::Below(mon),
            4 => Configure::Above(mon),
            5 => Configure::Zero,
            _ => unreachable!(),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Configure::Mirror(_) => "edit-copy",
            Configure::LeftOf(_) => "go-previous",
            Configure::RightOf(_) => "go-next",
            Configure::Below(_) => "go-down",
            Configure::Above(_) => "go-up",
            Configure::Zero => "go-home",
        }
    }
}

impl<'a> fmt::Display for Configure<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Configure::Mirror(_) => "Mirror",
                Configure::LeftOf(_) => "Left of",
                Configure::RightOf(_) => "Right of",
                Configure::Below(_) => "Below",
                Configure::Above(_) => "Above",
                Configure::Zero => "Zero",
            }
        )
    }
}

impl<'a> From<&Configure<'a>> for u64 {
    fn from(configure: &Configure) -> u64 {
        match configure {
            Configure::Mirror(_) => 0,
            Configure::LeftOf(_) => 1,
            Configure::RightOf(_) => 2,
            Configure::Below(_) => 3,
            Configure::Above(_) => 4,
            Configure::Zero => 5,
        }
    }
}

pub trait Randr {
    fn get_monitors(&self) -> Vec<Monitor>;
    fn configure(&self, mon: &Monitor, config: Configure);
}
