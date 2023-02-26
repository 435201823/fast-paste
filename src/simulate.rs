use std::thread::sleep;
use std::time::Duration;
use winput::Vk;


pub struct Simulate{}

impl Simulate {
    pub fn ctrl_c(){
        unsafe {
            sleep(Duration::from_millis(100));
            winput::press(Vk::Control);
            winput::press(Vk::C);
            winput::release(Vk::C);
            winput::release(Vk::Control);
            sleep(Duration::from_millis(100));
        }
    }

    pub fn ctrl_v(){
        unsafe {
            sleep(Duration::from_millis(100));
            winput::press(Vk::Control);
            winput::press(Vk::V);
            winput::release(Vk::V);
            winput::release(Vk::Control);
            sleep(Duration::from_millis(100));
        }
    }

}