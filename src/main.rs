use clap::{arg, Command};
use std::io::{stdin, stdout, Write};

pub mod kill;
pub mod lib;

// const VERSION: &'static str = "";
// const APP_NAME: &'static str = "Kouik";

fn build_cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ferry Jérémie ferryjeremie@free.fr")
        .author("Gabriel Theuws gaby.theuws@gmail.com")
        .about("kill program")
        .arg(arg!([program] "nom du programme à tuer"))
        .arg_required_else_help(true)
}

fn main() {
    let matches = build_cli().get_matches();

    if let Some(program_name) = matches.value_of("program") {
        let procs: Result<Vec<lib::Proc>, std::io::Error> = lib::get_procs();

        match procs {
            Ok(liste_procs) => {
                /* find exact programme */
                let res_nb_killed_or_trace: Result<u32, String> =
                    kill::kill_proc_by_name(program_name, &liste_procs);

                if let Err(error_trace) = res_nb_killed_or_trace {
                    eprintln!("{}", error_trace);
                    return;
                }

                let nb_killed = res_nb_killed_or_trace.unwrap();

                if nb_killed == 0 {
                    // calcul leveinstein distance pour tous
                    let proc_with_levensthein_distance =
                        lib::obtain_levensthein_distance(program_name, liste_procs);

                    let value_max_to_be_close: usize = program_name.chars().count() / 2 - 1;

                    /* Si la valeur est en dessous ou égale à zero ça signifirai que
                     * le nom est considéré proche s'il correspond parfaitement (distance levensthein == 0)
                     * et si nous sommes là c'est qu'il n'y en avait aucun qui correspondait parfaitement
                     */
                    if value_max_to_be_close == 0 {
                        println!("Aucun processus ne correspond au nom {:?}", program_name);
                        return;
                    }

                    let mut processus_similar = Vec::<lib::Proc>::new();

                    for processus in proc_with_levensthein_distance {
                        if processus.levensthein_distance <= value_max_to_be_close {
                            processus_similar.push(processus.proc);
                        }
                    }

                    let accept_choice =
                        interact_with_user_ask_if_it_must_kill(program_name, &processus_similar);

                    if let Some(accepted_index) = accept_choice {
                        if let Some(procs) = processus_similar.get(accepted_index) {
                            if let Some(error_description) = kill::kill_proc(procs) {
                                eprintln!("Une erreur s'est produite  : {}", error_description);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("flûte une erreur s'est produite ! : {:?}", e);
            }
        }
    }
}

fn interact_with_user_ask_if_it_must_kill(
    progname: &str,
    processus_similar: &[lib::Proc],
) -> Option<usize> {
    match processus_similar.len() {
        /* if there are one programme say Yes or No */
        1 => {
            println!(
                "Un processus au nom similaire à été trouvé pour \"{}\"",
                progname
            );
            print!(
                "Voulez vous tuer le processus {:?} ? (o/N)\t",
                processus_similar.get(0).unwrap().name
            );
            stdout().flush().expect("Le flush de stdout à échoué");
            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if s == "O\n" || s == "o\n" {
                return Some(0);
            }
            None
        }
        /* if no programme find, then send error message */
        0 => {
            println!("Aucun processus trouvé pour le nom \"{}\"", progname);
            None
        }
        /* if there are several programme, ask for user correction or choose ???
         * Est-ce qu'il doit se reformuler ou alors il choisi dans la liste ???
         */
        _ => {
            println!(
                "Plusieurs correspondances trouvées {:?} ou {:?} ?",
                processus_similar.get(0).unwrap().name,
                processus_similar.get(1).unwrap().name
            );
            None // a implémenter
        }
    }
}
