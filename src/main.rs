use chrono::{DateTime, ParseError, Utc};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::layout::Alignment;
use ratatui::text::Text;
use ratatui::widgets::{List, ListItem};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::io::{self, stdout};

#[derive(Debug, Serialize, Deserialize)]
struct GitHubOwner {
    login: String,
    html_url: String,
    avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {
    id: u64,
    name: String,
    full_name: String,
    html_url: String,
    description: Option<String>,
    forks_count: u64,
    stargazers_count: u64,
    owner: GitHubOwner,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubSearchResult {
    total_count: u64,
    incomplete_results: bool,
    items: Vec<GitHubRepo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitLabNamespace {
    id: u64,
    name: String,
    path: String,
    kind: String,
    full_path: String,
    web_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitLabProject {
    id: u64,
    name: String,
    path_with_namespace: String,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    web_url: String,
    readme_url: Option<String>,
    forks_count: u64,
    star_count: u64,
    namespace: GitLabNamespace,
}

#[derive(Clone, Copy, PartialEq)]
enum SearchEngine {
    GitHub,
    GitLab,
    Bitbucket,
    Wikipedia,
}

#[derive(Clone, Copy, PartialEq)]
enum SearchCategory {
    Repositories,
    Users,
    Topics,
}

impl Display for SearchCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repositories => write!(f, "Repositories"),
            Self::Users => write!(f, "Users"),
            Self::Topics => write!(f, "Topics"),
        }
    }
}

impl Display for SearchEngine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHub => write!(f, "GitHub"),
            Self::GitLab => write!(f, "GitLab"),
            Self::Bitbucket => write!(f, "BitBucket"),
            Self::Wikipedia => write!(f, "Wikipedia"),
        }
    }
}
impl SearchEngine {
    fn base_url(&self, category: SearchCategory) -> &'static str {
        match self {
            SearchEngine::GitHub => match category {
                SearchCategory::Repositories => "https://api.github.com/search/repositories?q=a",
                SearchCategory::Users => "https://api.github.com/search/users?q=",
                SearchCategory::Topics => "https://api.github.com/search/topics?q=",
            },
            SearchEngine::GitLab => match category {
                SearchCategory::Repositories => "https://gitlab.com/api/v4/projects?search=",
                SearchCategory::Users => "https://gitlab.com/api/v4/users?search=",
                SearchCategory::Topics => "",
            },
            SearchEngine::Bitbucket => match category {
                SearchCategory::Repositories => {
                    "https://api.bitbucket.org/2.0/repositories?q=name~"
                }
                SearchCategory::Users => "https://api.bitbucket.org/2.0/users?q=username~",
                SearchCategory::Topics => "",
            },
            SearchEngine::Wikipedia => {
                "https://en.wikipedia.org/w/api.php?action=query&list=search&format=json&srsearch="
            }
        }
    }
}

struct App {
    input: String,
    category: Vec<SearchCategory>,
    results: Vec<String>,
    results_to_display: Vec<String>,
    scroll_offset: usize,
    engine: Vec<SearchEngine>,
    current_engine: usize,
    selected_category: usize,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            category: vec![
                SearchCategory::Topics,
                SearchCategory::Repositories,
                SearchCategory::Users,
            ],
            results: vec![],
            results_to_display: vec![],
            scroll_offset: 0,
            engine: vec![
                SearchEngine::GitHub,
                SearchEngine::GitLab,
                SearchEngine::Bitbucket,
                SearchEngine::Wikipedia,
            ],
            current_engine: 0,
            selected_category: 0,
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn scroll_down(&mut self) {
        if (self.scroll_offset as usize) < self.results.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }
    fn save_github_repositories(&mut self, item: &Value) {
        let data = item
            .get("updated_at")
            .expect("")
            .as_str()
            .unwrap_or("No date");

        let created_at = item
            .get("created_at")
            .expect("")
            .as_str()
            .unwrap_or("No date");

        let time = ago(data).expect("");
        let creat_time = ago(created_at).expect("");
        let open_issues = item
            .get("open_issues")
            .expect("")
            .as_number()
            .expect("")
            .to_string();

        let description = item
            .get("description")
            .expect("")
            .as_str()
            .unwrap_or("No description");
        let watchers = item
            .get("watchers")
            .expect("")
            .as_number()
            .expect("")
            .to_string();
        let owner = item
            .get("owner")
            .expect("")
            .as_object()
            .expect("")
            .get("login")
            .unwrap()
            .as_str()
            .expect("")
            .to_string();

        let forks = item.get("forks").expect("").as_str().unwrap_or("0");

        let clone = item
            .get("html_url")
            .expect("")
            .as_str()
            .unwrap_or("No description");

        let stargazers_count = item
            .get("stargazers_count")
            .expect("")
            .as_number()
            .expect("")
            .to_string();

        self.results_to_display.push(format!(
            "repo owner : {owner}\ndate begin : {creat_time}\nlast modif : {time}\nopen issue : {open_issues}\nrepo stars : {stargazers_count}\nrepo forks : {forks}\nrepo watch : {watchers}\nrepo clone : {clone}\nrepo descr : {description}\n\n"));
    }

    fn save_gitlab_repositories(&mut self, item: &Value) {
        let data = item
            .get("last_activity_at")
            .expect("")
            .as_str()
            .unwrap_or("No date");

        let created_at = item
            .get("created_at")
            .expect("")
            .as_str()
            .unwrap_or("No date");

        let time = ago(data).expect("");
        let creat_time = ago(created_at).expect("");
        let open_issues = item
            .get("open_issues")
            .expect("")
            .as_number()
            .expect("")
            .to_string();

        let description = item
            .get("description")
            .expect("")
            .as_str()
            .unwrap_or("No description");
        let watchers = item
            .get("watchers")
            .expect("")
            .as_number()
            .expect("")
            .to_string();
        let owner = item
            .get("namespace")
            .expect("")
            .as_object()
            .expect("")
            .get("name")
            .unwrap()
            .as_str()
            .expect("")
            .to_string();

        let forks = item.get("forks_count").expect("").as_str().unwrap_or("0");

        let clone = item
            .get("http_url_to_repo")
            .expect("")
            .as_str()
            .unwrap_or("No description");

        let stargazers_count = item
            .get("star_count")
            .expect("")
            .as_number()
            .expect("")
            .to_string();

        self.results_to_display.push(format!(
            "repo owner : {owner}\ndate begin : {creat_time}\nlast modif : {time}\nopen issue : {open_issues}\nrepo stars : {stargazers_count}\nrepo forks : {forks}\nrepo watch : {watchers}\nrepo clone : {clone}\nrepo descr : {description}\n\n"));
    }
    fn save(&mut self, search: SearchEngine, category: SearchCategory, item: &Value) {
        match search {
            SearchEngine::GitHub => match category {
                SearchCategory::Repositories => self.save_github_repositories(item),
                SearchCategory::Users => {}
                SearchCategory::Topics => {}
            },
            SearchEngine::GitLab => match category {
                SearchCategory::Repositories => self.save_gitlab_repositories(item),
                SearchCategory::Users => {}
                SearchCategory::Topics => {}
            },
            SearchEngine::Bitbucket => {}
            SearchEngine::Wikipedia => {}
        }
    }

    fn search(&mut self) {
        self.results_to_display.clear();
        let engine = self.engine[self.current_engine];
        let category = self.category.get(self.selected_category).expect("").clone();
        let client = Client::new();
        let url = format!("{}{}", engine.base_url(category), self.input);

        match engine {
            SearchEngine::GitHub => {}
            SearchEngine::GitLab => {}
            SearchEngine::Bitbucket => {}
            SearchEngine::Wikipedia => {}
        }

        if let Ok(resp) = client
            .get(&url)
            .header("Accept", "application/json")
            .header("User-Agent", "awq")
            .send()
        {
            if let Ok(json) = resp.json::<Value>() {
                if let Some(items) = json["items"].as_array() {
                    self.results_to_display.push(format!(
                        "resp match : {}",
                        json["total_count"].as_number().expect("").to_string()
                    ));
                    for item in items {
                        self.save(engine, category, &item);
                    }
                } else if let Some(items) = json.as_array() {
                    for item in items {
                        self.save(engine, category, &item);
                    }
                }
            }
        }
    }
}

pub fn ago(date_str: &str) -> Result<String, ParseError> {
    let datetime: DateTime<Utc> = date_str.parse()?;
    let now = Utc::now();
    let duration = now.signed_duration_since(datetime);

    let result = if duration.num_seconds() < 60 {
        format!("{} seconds ago", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{} minutes ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_days() < 30 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_weeks() < 52 {
        format!("{} weeks ago", duration.num_weeks())
    } else {
        format!("{} years ago", duration.num_days() / 365)
    };

    Ok(result)
}
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();
    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            let areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                ])
                .split(chunks[0]);

            let engine_block = Block::default()
                .title(" Engine ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let engine_category_block = Block::default()
                .title(" Category ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let results_blocks = Block::default()
                .title(" Results ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let input_block = Block::default()
                .title(" Search ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL);
            let input_paragraph = Paragraph::new(app.input.clone()).block(input_block);
            let engine_category_paragraph = Paragraph::new(
                app.category
                    .get(app.selected_category)
                    .expect("")
                    .to_string(),
            )
            .centered()
            .block(engine_category_block);

            let engine_paragraph =
                Paragraph::new(app.engine.get(app.current_engine).expect("").to_string())
                    .centered()
                    .block(engine_block);
            f.render_widget(engine_paragraph, areas[0]);
            f.render_widget(engine_category_paragraph, areas[1]);
            f.render_widget(input_paragraph, areas[2]);

            let repo_items: Vec<ListItem> = app
                .results_to_display
                .iter()
                .map(|repo| ListItem::new(Text::from(repo.to_string())))
                .collect();

            let list = List::new(repo_items.clone()).block(results_blocks);
            f.render_widget(list, chunks[1]);
        })?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => {
                    app.current_engine += 1;
                    if app.current_engine.ge(&app.engine.len()) {
                        app.current_engine = 0;
                    }
                }
                KeyCode::BackTab => {
                    app.selected_category += 1;
                    if app.selected_category.ge(&3) {
                        app.selected_category = 0;
                    }
                }
                KeyCode::Up => app.scroll_up(),
                KeyCode::Down => app.scroll_down(),
                KeyCode::Enter => app.search(),
                KeyCode::Char(x) => app.input.push(x),
                KeyCode::Esc => break,
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
        }
    }
    terminal.clear()?;
    disable_raw_mode()?;
    Ok(())
}
