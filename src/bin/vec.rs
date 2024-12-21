use std::{thread::sleep, time::Duration, vec};

#[tokio::main]
async fn main() {
    let school = School {
        sorcers: vec![
            Sorcer {
                name: "harry".to_string(),
                student: false,
            }
            .to_owned(),
            Sorcer {
                name: "ron".to_string(),
                student: false,
            }
            .to_owned(),
            Sorcer {
                name: "hermione".to_string(),
                student: false,
            }
            .to_owned(),
        ],
    };

    loop {
        let mut futures = vec![];

        for mut sorcer in school.sorcers.to_vec() {
            futures.push(tokio::spawn(async move {
                sorcer.go_to_school().await;
                if sorcer.student {
                    println!("{:?} is in school", sorcer.name);
                } else {
                    println!("{:?} is NOT in school", sorcer.name);
                }
            }));
        }

        for f in futures {
            let _ = f.await;
        }

        sleep(Duration::from_secs(1));
    }
}

#[derive(Debug, Clone)]
struct Sorcer {
    name: String,
    student: bool,
}

impl Sorcer {
    async fn go_to_school(&mut self) {
        println!("{:?} go to school", self.name);
        sleep(Duration::from_secs(3));
        self.student = !self.student;
    }
}

#[derive(Debug, Clone)]
struct School {
    sorcers: Vec<Sorcer>,
}
