use std::io::{self, stdout};
use std::process::Command;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

#[derive(Debug)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, PartialEq)]
enum PackageManager {
    Pkg,
    Apt,
    Pip,
}

struct PackageList {
    items: Vec<Package>,
    state: ListState,
    package_manager: PackageManager,
}

impl PackageList {
    fn load(package_manager: PackageManager) -> Self {
        let output = match package_manager {
            PackageManager::Pkg => Command::new("pkg")
                .arg("list-installed")
                .output()
                .expect("Failed to execute pkg list-installed"),
            PackageManager::Apt => Command::new("apt")
                .arg("list")
                .arg("--installed")
                .output()
                .expect("Failed to execute apt list --installed"),
            PackageManager::Pip => Command::new("pip")
                .arg("list")
                .output()
                .expect("Failed to execute pip list"),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let items: Vec<Package> = stdout
            .lines()
            .filter_map(|line| {
                match package_manager {
                    PackageManager::Pkg => {
                        let parts: Vec<&str> = line.split('/').collect();
                        if parts.len() >= 2 {
                            Some(Package {
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                            })
                        } else {
                            None
                        }
                    }
                    PackageManager::Apt => {
                        let parts: Vec<&str> = line.split('/').collect();
                        if parts.len() >= 2 {
                            let version_parts: Vec<&str> = parts[1].split(' ').collect();
                            if version_parts.len() >= 1 {
                                Some(Package {
                                    name: parts[0].to_string(),
                                    version: version_parts[0].to_string(),
                                })
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    PackageManager::Pip => {
                        if line.contains("Package") || line.contains("---") {
                            return None; // Skip header lines
                        }
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            Some(Package {
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                            })
                        } else {
                            None
                        }
                    }
                }
            })
            .collect();

        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self {
            items,
            state,
            package_manager,
        }
    }

    fn toggle_package_manager(&mut self) {
        let new_manager = match self.package_manager {
            PackageManager::Pkg => PackageManager::Apt,
            PackageManager::Apt => PackageManager::Pip,
            PackageManager::Pip => PackageManager::Pkg,
        };
        *self = Self::load(new_manager);
    }

    fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_first(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        }
    }

    fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(self.items.len().saturating_sub(1)));
        }
    }

    fn fetch_package_details(&self, package_name: &str) -> String {
        let output = match self.package_manager {
            PackageManager::Pkg => Command::new("pkg")
                .arg("show")
                .arg(package_name)
                .output(),
            PackageManager::Apt => Command::new("apt")
                .arg("show")
                .arg(package_name)
                .output(),
            PackageManager::Pip => Command::new("pip")
                .arg("show")
                .arg(package_name)
                .output(),
        };

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.is_empty() {
                    "No details available".to_string()
                } else {
                    stdout.to_string()
                }
            }
            Err(_) => "Failed to fetch package details".to_string(),
        }
    }
}

struct App {
    should_exit: bool,
    package_list: PackageList,
}

impl App {
    fn new() -> Self {
        Self {
            should_exit: false,
            package_list: PackageList::load(PackageManager::Pkg),
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.should_exit = true;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.package_list.select_next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.package_list.select_previous();
                        }
                        KeyCode::Home | KeyCode::Char('g') => {
                            self.package_list.select_first();
                        }
                        KeyCode::End | KeyCode::Char('G') => {
                            self.package_list.select_last();
                        }
                        KeyCode::Tab => {
                            self.package_list.toggle_package_manager();
                        }
                        _ => {}
                    }
                }
            }

            if self.should_exit {
                return Ok(());
            }
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(f.size());

        let list_area = chunks[0];
        let detail_area = chunks[1];

        // Render package list
        let items: Vec<ListItem> = self
            .package_list
            .items
            .iter()
            .map(|pkg| ListItem::new(format!("{} {}", pkg.name, pkg.version)))
            .collect();

        let title = match self.package_list.package_manager {
            PackageManager::Pkg => "Installed Packages (pkg)",
            PackageManager::Apt => "Installed Packages (apt)",
            PackageManager::Pip => "Installed Packages (pip)",
        };

        let list = List::new(items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, list_area, &mut self.package_list.state);

        // Render selected package details
        let detail = if let Some(i) = self.package_list.state.selected() {
            let pkg = &self.package_list.items[i];
            self.package_list.fetch_package_details(&pkg.name)
        } else {
            "No package selected".to_string()
        };

        let paragraph = Paragraph::new(detail)
            .block(Block::default().title("Package Details").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, detail_area);
    }
}
