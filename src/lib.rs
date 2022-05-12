extern crate levenshtein;
use levenshtein::levenshtein;
use std::fs;
use std::path::Path;
use std::io::Error;
use std::fs::File;
use std::io::Read;

pub struct Proc {
	pub pid: i32,
	pub name: String,
}

pub fn get_procs() -> Result<Vec<Proc>,Error> {

	let mut processus = Vec::<Proc>::new();
    
	for entry in fs::read_dir(Path::new("/proc"))? {

		let entry = entry?;
		let path = entry.path().join("stat");
		
        // println!("le path est : {}", path.as_path().to_str().unwrap());
        
		if path.exists() && !path.is_dir() {
            
            let mut contents = String::new();
            
            File::open(path)?.read_to_string(&mut contents)?;
            
            let start_paren = contents.find('(').unwrap();
            let end_paren = contents.rfind(')').unwrap();
            let pid_s = &contents[..start_paren - 1];
            let name = contents[start_paren + 1..end_paren].to_string();

            processus.push(Proc { pid: pid_s.parse().unwrap(), name});
        }
	}
	Ok(processus)
}

pub struct LevensteinProc {
	pub proc : Proc,
	pub levensthein_distance :usize,
}

pub fn obtain_levensthein_distance(progname: &str, liste_procs: Vec<Proc>) -> Vec<LevensteinProc> {
	
	let mut leven_vec_of_proc = Vec::<LevensteinProc>::new();
    
	for processus in liste_procs {

		let mut min: usize = 999;
        
        let dist = levenshtein(&processus.name,progname);
        if dist < min {
            min = dist;
        }

		leven_vec_of_proc.push(LevensteinProc{proc: processus, levensthein_distance: min});
	}

	leven_vec_of_proc
}

