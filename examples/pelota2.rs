use core::time::Duration;

use std::sync::Arc;//pointeur permet de partager des donnes entre plusieurs endroits

use esp_idf_svc::log::EspLogger;//active log
use esp_idf_svc::sys::EspError;
use esp_idf_svc::timer::EspTaskTimerService;//creation de timer
use esp_idf_svc::hal::task::block_on;

use core::pin::pin;

use log::info;

fn wait_until(t_next:u64)-> Result<(), EspError> {
    let timer_service = EspTaskTimerService::new()?;//creation du timer

    block_on(async{
        let mut timer = timer_service.timer_async()?;
        timer.after(Duration::from_secs(t_next)).await?;
        Ok(())//ligne d efin du bloc async qui dit que tout s'est bien passe

    })
}

fn main() {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    log::info!("Hello, world!");
    wait_until(2).unwrap();
    log::info!("After 2 secondes, goodbye World!");

}