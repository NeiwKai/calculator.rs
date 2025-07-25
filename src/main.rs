use eframe::*;
use egui::CentralPanel;
use egui::Ui;

#[derive(PartialEq)]
enum Operation {
    NON,
    ADD,
    SUB,
    MPY,
    DIV,
    MOD
}

struct MyApp {
    current_var: f32,
    new_var: f32,
    display_var: String,
    operation: Operation,
    reset: bool
}

fn operate(x: &f32, operand: &Operation, y: &f32) -> f32 {
    match operand {
        Operation::NON => return 0.0,
        Operation::ADD => return x+y,
        Operation::SUB => return x-y,
        Operation::MPY => return x*y,
        Operation::DIV => return x/y,
        Operation::MOD => {
            if *y==0.0 {
                return x/100.0;
            }
            return x%y;
        },
    }
}

pub struct Grid {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label(&self.display_var);
            egui::Grid::new("button").show(ui, |ui| {
                if ui.button("AC").clicked() {
                    self.current_var = 0.0;
                    self.new_var = 0.0;
                    self.display_var = String::from("0");
                    self.operation = Operation::NON;
                    self.reset = false;
                }
                if ui.button("+/-").clicked() && self.display_var != "0" {
                    if self.display_var.chars().nth(0) != Some('-') {
                        self.display_var = format!("-{}", self.display_var); 
                    } else {
                        self.display_var.remove(0); 
                    }
                }
                if ui.button("%").clicked() {
                    if self.operation != Operation::NON {
                        self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                        self.new_var = 0.0;
                        self.display_var = self.current_var.to_string();
                        self.reset = false;
                    }
                    self.operation = Operation::MOD;
                }
                if ui.button("/").clicked() {
                    if self.operation != Operation::NON {
                        self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                        self.new_var = 0.0;
                        self.display_var = self.current_var.to_string();
                        self.reset = false;
                    }
                    self.operation = Operation::DIV;
                }
                ui.end_row();
                if ui.button("7").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('7');
                        } else {
                            self.display_var = String::from("7");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("7");
                            self.new_var = 7.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('7');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("7");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("8").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('8');
                        } else {
                            self.display_var = String::from("8");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("8");
                            self.new_var = 8.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('8');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("8");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("9").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('9');
                        } else {
                            self.display_var = String::from("9");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("9");
                            self.new_var = 9.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('9');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("9");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("*").clicked() {
                    if self.operation != Operation::NON {
                        self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                        self.new_var = 0.0;
                        self.display_var = self.current_var.to_string();
                        self.reset = false;
                    }
                    self.operation = Operation::MPY;
                }
                ui.end_row();
                if ui.button("4").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('4');
                        } else {
                            self.display_var = String::from("4");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("4");
                            self.new_var = 4.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('4');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("4");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("5").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('5');
                        } else {
                            self.display_var = String::from("5");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("5");
                            self.new_var = 5.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('5');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("5");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("6").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('6');
                        } else {
                            self.display_var = String::from("6");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("6");
                            self.new_var = 6.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('6');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("6");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("-").clicked() {
                    if self.operation != Operation::NON {
                        self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                        self.new_var = 0.0;
                        self.display_var = self.current_var.to_string();
                        self.reset = false;
                    }
                    self.operation = Operation::SUB;
                }
                ui.end_row();
                if ui.button("1").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('1');
                        } else {
                            self.display_var = String::from("1");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("1");
                            self.new_var = 2.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('1');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("1");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("2").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('2');
                        } else {
                            self.display_var = String::from("2");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("2");
                            self.new_var = 2.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('2');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("2");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("3").clicked() {
                    if self.operation == Operation::NON {
                        if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('3');
                        } else {
                            self.display_var = String::from("3");
                        }
                        self.current_var = self.display_var.parse::<f32>().unwrap();
                    } else {
                        if !self.reset {
                            self.reset = true;
                            self.display_var = String::from("3");
                            self.new_var = 3.0;
                        } else if self.display_var != "0" || self.display_var.find('.').is_some() {
                            self.display_var.push('3');
                            self.new_var = self.display_var.parse::<f32>().unwrap();
                        } else {
                            self.display_var = String::from("3");
                        }
                        self.new_var = self.display_var.parse::<f32>().unwrap();
                    }
                }
                if ui.button("+").clicked() {
                    if self.operation != Operation::NON {
                        self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                        self.new_var = 0.0;
                        self.display_var = self.current_var.to_string();
                        self.reset = false;
                    }
                    self.operation = Operation::ADD;
                }
                ui.end_row();
                if ui.button("").clicked() && self.operation != Operation::NON {
                }
                if ui.button("0").clicked() {
                    if self.operation != Operation::NON {
                        if !self.reset {
                            self.display_var = String::from("0");
                            self.reset = true;
                        } else if self.display_var.find('.').is_some() && self.reset {
                            self.display_var.push('0');
                        }
                    } else {
                        if self.display_var.find('.').is_some() {
                            self.display_var.push('0');
                        } else {
                            self.display_var = String::from("0");
                        }
                    }
                }
                if ui.button(".").clicked() {
                    if !self.display_var.find('.').is_some() {
                        self.display_var.push('.'); 
                    }
                }
                if ui.button("=").clicked() && self.operation != Operation::NON {
                    self.current_var = operate(&self.current_var, &self.operation, &self.new_var);
                    self.new_var = 0.0;
                    self.display_var = self.current_var.to_string();
                    self.operation = Operation::NON;
                    self.reset = false;
                }
                ui.end_row();
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    run_native(
        "My App",
        NativeOptions::default(),
        Box::new(|_cc: &CreationContext<'_>| {
            Ok(Box::new(MyApp {current_var: 0.0, new_var: 0.0, display_var: String::from("0"), operation: Operation::NON, reset: false}))
        })
    )
}
