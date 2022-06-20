

use bsc::{
    led::{RGB8, WS2812RMT},
    temp_sensor::BoardTempSensor,
    wifi::wifi,
};


use esp_idf_hal;
use esp_idf_svc; 
use esp_idf_sys; 

const UUID: &'static str = get_uuid::uuid();


fn main() -> anyhow::Result<()> {

    e№sp_idf_sys::link_patches();





    Ok(());
}