use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::GetAsyncKeyState;
use winput::Vk;

pub struct Simulate {}

impl Simulate {
    pub fn ctrl_c() {
        unsafe {
            sleep(Duration::from_millis(500));
            winput::press(Vk::Control);
            winput::press(Vk::C);
            winput::release(Vk::C);
            winput::release(Vk::Control);
            sleep(Duration::from_millis(500));
        }
    }

    pub fn ctrl_v() {
        unsafe {
            sleep(Duration::from_millis(100));
            winput::press(Vk::Control);
            winput::press(Vk::V);
            winput::release(Vk::V);
            winput::release(Vk::Control);
            sleep(Duration::from_millis(100));
        }
    }

    pub fn wait_no_key() {
        loop {
            let state = unsafe { GetAsyncKeyState(-1) };
            if state == 0 {
                println!("没有按键按下");
                break;
            }
            println!("有按键按下")
        }
    }
}
