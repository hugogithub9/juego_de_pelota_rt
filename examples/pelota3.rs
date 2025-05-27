use core::time::Duration;

use std::sync::Arc;//pointeur permet de partager des donnes entre plusieurs endroits

use esp_idf_svc::log::EspLogger;//active log
use esp_idf_svc::sys::EspError;
use esp_idf_svc::timer::EspTaskTimerService;//creation de timer
use esp_idf_svc::hal::task::block_on;
use core::pin::pin;


mod player1 {
    use super:: wait_until;
    use log::info;
    pub struct Player1State {
        sigma: f64,
        period: f64,
        clock:f64,
        count: usize,
    }

    impl Player1State {
        pub fn new(period: f64) -> Self {
            Self {
                sigma: 0.0,
                period,
                clock: 0.0,
                count: 0,
            }
        }
    }

    xdevs::component!(
        ident = Player1,
        input = {
            receive_pelota1<bool>,
        },
        output = {
            send_pelota1<bool>,
        },
        state = Player1State,
    );

    impl xdevs::Atomic for Player1 {
        fn delta_int(state: &mut Self::State) { 
            state.count += 1;
            state.clock += state.sigma;
            state.sigma = f64::INFINITY;
            println!("[t={}]: player1 is waiting", state.clock);
        }

        fn lambda(state: &Self::State, output: &mut Self::Output) {
            wait_until(state.period);
            println!("[t={}]: player1 hits the pelota", state.clock+state.sigma);
            output.send_pelota1.add_value(true).unwrap();
        }

        fn ta(state: &Self::State) -> f64 {
            state.sigma
        }

        fn delta_ext(state: &mut Self::State, e: f64, input: &Self::Input) {
            state.sigma -= e;
            state.clock += e;
            if !input.receive_pelota1.is_empty() {
                state.sigma=state.period;
                println!("[t={}]: player1 receives the pelota", state.clock);
            }
        }
    }
}

mod player2 {
    use super::wait_until;
    use log::info;
    pub struct Player2State {
        sigma: f64,
        period: f64,
        clock:f64,
        count: usize,
    }

    impl Player2State {
        pub fn new(period: f64) -> Self {
            Self {
                sigma: f64::INFINITY,
                period,
                clock: 0.0,
                count: 0,
            }
        }
    }

    xdevs::component!(
        ident = Player2,
        input = {
            receive_pelota2<bool>,
        },
        output = {
            send_pelota2<bool>,
        },
        state = Player2State,
    );

    impl xdevs::Atomic for Player2 {
        fn delta_int(state: &mut Self::State) { //je ne peux pas envoyer le message player2 is waiting dans delta int
            state.count += 1;
            state.clock += state.sigma;
            state.sigma = f64::INFINITY; 
            println!("[t={}]: player2 is waiting", state.clock);
        }

        fn lambda(state: &Self::State, output: &mut Self::Output) {
            wait_until(state.period);
            println!("[t={}]: player2 hits the pelota", state.clock+state.sigma);
            output.send_pelota2.add_value(true).unwrap();
        }

        fn ta(state: &Self::State) -> f64 {
            state.sigma
        }

        fn delta_ext(state: &mut Self::State, e: f64, input: &Self::Input) {
            state.sigma -= e;
            state.clock += e;
            if !input.receive_pelota2.is_empty() {
                state.sigma=state.period;
                println!("[t={}]: player2 receives the pelota", state.clock);
            }
        }
    }
}

enum Target { //énumère les cibles de pelota 

    P1,
    P2,
}

mod pelota {
    use super::wait_until;
    use log::info;
    use super::Target;
    pub struct PelotaState {
        sigma: f64,
        travel: f64,
        clock:f64,
        count: usize,
        next_target: Option<Target>,
    }

    impl PelotaState {
        pub fn new(travel: f64) -> Self {
            Self {
                sigma: 0.0,
                travel,
                clock: 0.0,
                count: 0,
                next_target:None, 
            }
        }
    }

    xdevs::component!(
        ident = Pelota,
        input = {
            touch_P1<bool>,
            touch_P2<bool>,
        },
        output = {
            to_P1<bool>,
            to_P2<bool>,
        },
        state = PelotaState,
    );

    impl xdevs::Atomic for Pelota {
        fn delta_int(state: &mut Self::State) { 
            state.count += 1;
            state.clock += state.sigma;
            state.sigma = f64::INFINITY; 
        }

        fn lambda(state: &Self::State, output: &mut Self::Output) {
            match state.next_target{ //regarde valeur de next_target
            Some(Target::P1)=>{ //si la balle doit aller en P1 alors add_value to P1
                wait_until(state.travel);
                output.to_P1.add_value(true); 
            },
            Some(Target::P2)=>{
                wait_until(state.travel);
                output.to_P2.add_value(true); 
            },
            None => {}//si aucune cible alors fait rien
            }
        }

        fn ta(state: &Self::State) -> f64 {
            state.sigma
        }

        fn delta_ext(state: &mut Self::State, e: f64, input: &Self::Input) {
        state.clock+=e;
        state.sigma-=e;
        state.count+=1;
        if !input.touch_P1.is_empty() {
            state.sigma=state.travel;
            state.next_target=Some(Target::P2); //definit la cible de la balle après avoir touché P1
            println!("[t={}]: the pelota is travelling to P2", state.clock);
        }
        if !input.touch_P2.is_empty() {
            state.sigma=state.travel;
            state.next_target=Some(Target::P1);
            println!("[t={}]: the pelota is travelling to P1", state.clock);
        }
        }
    }
}

xdevs::component!(
    ident = JuegoDePelota,
    components = {
        player1: player1::Player1,
        player2: player2::Player2,
        pelota: pelota::Pelota,
    },
    couplings = {
        pelota.to_P1 -> player1.receive_pelota1,
        pelota.to_P2 -> player2.receive_pelota2,
        player1.send_pelota1 -> pelota.touch_P1,
        player2.send_pelota2 -> pelota.touch_P2,
    }
);

fn wait_until(t_next:f64)-> Result<(), EspError> {
    let timer_service = EspTaskTimerService::new()?;//creation du timer

    block_on(async{
        let mut timer = timer_service.timer_async()?;
        timer.after(Duration::from_secs(t_next as u64)).await?;
        Ok(())//ligne de fin du bloc async qui dit que tout s'est bien passe

    })
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let period = 1.;
    let travel = 5.;
    println!("beginning of the game");

    let player1 = player1::Player1::new(player1::Player1State::new(period));
    let player2 = player2::Player2::new(player2::Player2State::new(period));
    let pelota = pelota::Pelota::new(pelota::PelotaState::new(travel));

    let juego = JuegoDePelota::new(player1, player2, pelota);

    let mut simulator = xdevs::simulator::Simulator::new(juego);
    simulator.simulate_vt( //start,stop
        0.0,
        60.0,
    );
}

