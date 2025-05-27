use core::time::Duration;

use std::sync::Arc;//pointeur permet de partager des donnes entre plusieurs endroits

use esp_idf_svc::log::EspLogger;//active log
use esp_idf_svc::sys::EspError;
use esp_idf_svc::timer::EspTaskTimerService;//creation de timer
use esp_idf_svc::hal::task::block_on;
use core::pin::pin;

use log::info;

fn main() {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    run().unwrap();
}

fn run() -> Result<(), EspError> { //fct renvoie soit ok(())=tout s'est bien passe, le () : pas devaleur utile a retourner OU une erreur type EspError
    let timer_service = EspTaskTimerService::new()?; //creer variable timer permet d'activer un compteur/ ?:si fct echoue retourne l'erreurer arrete

    block_on(pin!(async move{//lance tache asynchrone / move signfie que variable utilises sont deplace ds le bloc - garde memoire propre
                             //pin permet de garder ce bloc asynch ds la memoire/ block_on execute une tache asunc de maniere sync : att que la tache se terñine avant de continuer
        let mut timer = timer_service.timer_async()?; //creer une variable modifiable  on creer un timer asynchrone

        loop{ //boucle infinie
        timer.after(Duration::from_secs(1)).await?; //Duration::from_secs(1) : creer une duree de 1s
                                                    //timer.after(...).await : dit au programme : "attends pendant ce temps".
        info!("Player1 hits the pelota");//message de retour

        timer.after(Duration::from_secs(5)).await?;
        info!("Player2 receives the pelota");

        timer.after(Duration::from_secs(1)).await?;
        info!("Player2 hits the pelota");

        timer.after(Duration::from_secs(5)).await?;
        info!("Player1 receives the pelota");
        }

        Ok(())//ligne d efin du bloc async qui dit que tout s'est bien passe, ici boucle infini donc atteint jamais cette objectif

    }))
}
