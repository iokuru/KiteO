use std::collections::HashMap;

#[derive(Debug, Clone)]
enum MacroDef {
    Object(String),
    Function { params: Vec<String>, body: String },
}

pub struct Preprocessor {
    macros: HashMap<String, MacroDef>,
    type_aliases: HashMap<String, String>,
}

impl Default for Preprocessor {
    fn default() -> Self {
        let mut p = Self {
            macros: HashMap::new(),
            type_aliases: HashMap::new(),
        };
        p.register_standard_cp_macros();
        p
    }
}

impl Preprocessor {
    pub fn new() -> Self {
        Self::default()
    }

    fn register_standard_cp_macros(&mut self) {
        // Common standard aliases
        self.macros
            .insert("pb".to_string(), MacroDef::Object("push_back".to_string()));
        self.macros.insert(
            "eb".to_string(),
            MacroDef::Object("emplace_back".to_string()),
        );
        self.macros
            .insert("mp".to_string(), MacroDef::Object("make_pair".to_string()));
        self.macros
            .insert("fi".to_string(), MacroDef::Object("first".to_string()));
        self.macros
            .insert("se".to_string(), MacroDef::Object("second".to_string()));
        self.macros.insert(
            "sz(x)".to_string(),
            MacroDef::Function {
                params: vec!["x".to_string()],
                body: "(x).size()".to_string(),
            },
        );

        // Common typedefs
        self.type_aliases
            .insert("ll".to_string(), "long long".to_string());
        self.type_aliases
            .insert("ull".to_string(), "unsigned long long".to_string());
        self.type_aliases
            .insert("pii".to_string(), "pair<int, int>".to_string());
        self.type_aliases
            .insert("vi".to_string(), "vector<int>".to_string());
        self.type_aliases
            .insert("vll".to_string(), "vector<long long>".to_string());
        self.type_aliases
            .insert("vvi".to_string(), "vector<vector<int>>".to_string());
    }

    pub fn process(&mut self, source: &str) -> String {
        let mut cleaned_lines = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("#pragma") {
                continue;
            }

            if trimmed.starts_with("#include") {
                continue;
            }

            if trimmed.starts_with("#define") {
                self.parse_define(trimmed);
                continue;
            }

            if trimmed.starts_with("typedef") {
                self.parse_typedef(trimmed);
                continue;
            }

            if trimmed.starts_with("using") && trimmed.contains('=') && trimmed.ends_with(';') {
                self.parse_using(trimmed);
                continue;
            }

            cleaned_lines.push(line);
        }

        let mut content = cleaned_lines.join("\n");

        // Expand function-like macros first
        for (name, def) in &self.macros {
            if let MacroDef::Function { params, body } = def {
                content = expand_function_macro(&content, name, params, body);
            }
        }

        // Expand object-like macros and type aliases with word boundaries
        for (name, def) in &self.macros {
            if let MacroDef::Object(replacement) = def {
                content = replace_word(&content, name, replacement);
            }
        }

        for (alias, real_type) in &self.type_aliases {
            content = replace_word(&content, alias, real_type);
        }

        content
    }

    fn parse_define(&mut self, line: &str) {
        let rest = line["#define".len()..].trim();
        if rest.is_empty() {
            return;
        }

        // Standard C/C++: function-like macro has '(' immediately adjacent to the identifier with no whitespace
        let first_token = rest.split_whitespace().next().unwrap_or("");
        if let Some(open_paren) = first_token.find('(') {
            let name = first_token[..open_paren].trim().to_string();
            let after_open = &rest[name.len() + 1..];
            if let Some(close_paren) = after_open.find(')') {
                let params_str = &after_open[..close_paren];
                let params: Vec<String> = params_str
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect();
                let body = after_open[close_paren + 1..].trim().to_string();
                self.macros
                    .insert(name, MacroDef::Function { params, body });
                return;
            }
        }

        // Object-like macro
        let mut parts = rest.split_whitespace();
        if let Some(name) = parts.next() {
            let body = parts.collect::<Vec<_>>().join(" ");
            if name != "int" || !body.contains("long long") {
                self.macros.insert(name.to_string(), MacroDef::Object(body));
            }
        }
    }

    fn parse_typedef(&mut self, line: &str) {
        let trimmed = line.trim_end_matches(';').trim();
        let rest = trimmed["typedef".len()..].trim();
        if let Some(last_space) = rest.rfind(' ') {
            let original = rest[..last_space].trim();
            let alias = rest[last_space + 1..].trim();
            if !original.is_empty() && !alias.is_empty() {
                self.type_aliases
                    .insert(alias.to_string(), original.to_string());
            }
        }
    }

    fn parse_using(&mut self, line: &str) {
        let trimmed = line.trim_end_matches(';').trim();
        let rest = trimmed["using".len()..].trim();
        if let Some(eq_idx) = rest.find('=') {
            let alias = rest[..eq_idx].trim();
            let original = rest[eq_idx + 1..].trim();
            if !alias.is_empty() && !original.is_empty() {
                self.type_aliases
                    .insert(alias.to_string(), original.to_string());
            }
        }
    }
}

pub fn preprocess(source: &str) -> String {
    let mut p = Preprocessor::new();
    p.process(source)
}

fn expand_function_macro(source: &str, name: &str, params: &[String], body: &str) -> String {
    let mut result = String::new();
    let mut remaining = source;

    let target = format!("{name}(");

    while let Some(idx) = remaining.find(&target) {
        // Verify word boundary before macro name
        if idx > 0 {
            let prev = remaining.as_bytes()[idx - 1];
            if prev.is_ascii_alphanumeric() || prev == b'_' {
                result.push_str(&remaining[..idx + target.len()]);
                remaining = &remaining[idx + target.len()..];
                continue;
            }
        }

        result.push_str(&remaining[..idx]);
        let call_start = idx + target.len();

        // Find matching closing paren
        let mut depth = 1;
        let mut end_idx = call_start;
        let bytes = remaining.as_bytes();

        while end_idx < bytes.len() && depth > 0 {
            if bytes[end_idx] == b'(' {
                depth += 1;
            } else if bytes[end_idx] == b')' {
                depth -= 1;
            }
            if depth > 0 {
                end_idx += 1;
            }
        }

        if depth == 0 {
            let args_str = &remaining[call_start..end_idx];
            let args = split_macro_args(args_str);

            let mut expanded_body = body.to_string();
            for (param, arg) in params.iter().zip(args.iter()) {
                expanded_body = replace_word(&expanded_body, param, arg.trim());
            }

            result.push_str(&expanded_body);
            remaining = &remaining[end_idx + 1..];
        } else {
            result.push_str(&remaining[idx..call_start]);
            remaining = &remaining[call_start..];
        }
    }

    result.push_str(remaining);
    result
}

fn split_macro_args(args_str: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0;

    for c in args_str.chars() {
        match c {
            '(' | '<' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | '>' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                args.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.trim().is_empty() || !args.is_empty() {
        args.push(current.trim().to_string());
    }

    args
}

fn replace_word(text: &str, word: &str, replacement: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;

    while let Some(idx) = remaining.find(word) {
        let is_left_boundary = idx == 0
            || !remaining.as_bytes()[idx - 1].is_ascii_alphanumeric()
                && remaining.as_bytes()[idx - 1] != b'_';
        let right_idx = idx + word.len();
        let is_right_boundary = right_idx == remaining.len()
            || !remaining.as_bytes()[right_idx].is_ascii_alphanumeric()
                && remaining.as_bytes()[right_idx] != b'_';

        if is_left_boundary && is_right_boundary {
            result.push_str(&remaining[..idx]);
            result.push_str(replacement);
            remaining = &remaining[right_idx..];
        } else {
            result.push_str(&remaining[..right_idx]);
            remaining = &remaining[right_idx..];
        }
    }

    result.push_str(remaining);
    result
}
