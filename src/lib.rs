rofi_mode::export_mode!(Mode<'_>);
use serde::Deserialize;
use std::{process::Command, thread};

#[derive(Deserialize, Debug)]
struct OllamaModel {
    name: String,
}

#[derive(Deserialize, Debug)]
struct ListOllamaModel {
    models: Vec<OllamaModel>,
}

struct Mode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "ollama\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        let uri = String::from("http://127.0.0.1:11434/api/tags");
        let entries = match reqwest::blocking::get(uri) {
            Err(_) => vec![String::from("Is the server running ?")],
            Ok(data) => serde_json::from_slice::<ListOllamaModel>(&(data.bytes()).unwrap())
                .unwrap()
                .models
                .iter()
                .map(|m| m.name.clone())
                .collect(),
        };

        Ok(Self { api, entries })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        (&self.entries[line]).into()
    }

    fn entry_icon(&mut self, _line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        match self.api.query_icon("computer", height).wait(&mut self.api) {
            Ok(surface) => Some(surface),
            Err(_) => None,
        }
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        match event {
            rofi_mode::Event::Cancel { selected: _ } => return rofi_mode::Action::Exit,
            rofi_mode::Event::Ok {
                alt: false,
                selected,
            } => {
                let model = self.entries[selected].clone();
                //let prompt = input.clone();
                let prompt = "";
                let command = format!("ollama run {model} {prompt}");
                thread::spawn(move || {
                    Command::new("$TERMINAL")
                        .arg("--hold")
                        .arg("bash")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .expect("failed to execute process")
                });

                return rofi_mode::Action::Exit;
            }
            rofi_mode::Event::Ok { alt: true, .. } => {
                // let model = self.entries[selected].clone();
                // self.entries = vec![];
            }
            rofi_mode::Event::CustomInput {
                alt: false,
                selected: _,
            } => {}
            rofi_mode::Event::CustomInput {
                alt: true,
                selected: _,
            } => {
                self.api.replace_display_name(mem::take(input));
            }
            rofi_mode::Event::DeleteEntry { selected } => {
                self.entries.remove(selected);
            }
            rofi_mode::Event::Complete {
                selected: Some(selected),
            } => {
                input.clear();
                input.push_str(&self.entries[selected]);
            }
            rofi_mode::Event::Complete { .. } | rofi_mode::Event::CustomCommand { .. } => {}
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line])
    }

    fn message(&mut self) -> rofi_mode::String {
        let entries = self.entries.len();
        if entries == 1 {
            "1 entry registered".into()
        } else {
            rofi_mode::format!("{entries} entries registered")
        }
    }
}

use std::mem;
