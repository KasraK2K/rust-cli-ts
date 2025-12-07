use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};

pub struct TemplateFile {
    pub file_name: String,
    pub content: String,
    pub path: String,
}

#[allow(dead_code)]
pub enum Flag {
    All(String),
    Routes(String),
    Controller(String),
    Service(String),
    Logic(String),
    Repository(String),
    Interface(String),
    Schema,
    Unknown,
}

impl Flag {
    pub fn get_flag(kind: &str, name: Option<String>) -> Flag {
        if name == None {
            panic!("No name given");
        }

        match kind {
            "all" => Flag::All(name.unwrap_or_default()),
            "controller" => Flag::Controller(name.unwrap_or_default()),
            "routes" => Flag::Routes(name.unwrap_or_default()),
            "service" => Flag::Service(name.unwrap_or_default()),
            "logic" => Flag::Logic(name.unwrap_or_default()),
            "repository" => Flag::Repository(name.unwrap_or_default()),
            "interface" => Flag::Interface(name.unwrap_or_default()),
            "schema" => Flag::Schema,
            _ => Flag::Unknown,
        }
    }
}

fn get_content(template: &str, name: &str) -> String {
    let pascal_case = name.to_upper_camel_case();
    let camel_case = name.to_lower_camel_case();
    let lower_case = name.to_lowercase();
    let snake_case = name.to_snake_case();

    template
        .replace("{{PascalName}}", &pascal_case)
        .replace("{{camelName}}", &camel_case)
        .replace("{{lowerName}}", &lower_case)
        .replace("{{snakeCase}}", &snake_case)
        .replace("{{Name}}", name)
}

pub fn get_template_files(flag: &Flag, path: Option<String>) -> Vec<TemplateFile> {
    let path = path.unwrap_or_else(|| {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .into_owned()
    });

    match flag {
        Flag::All(name) => vec![
            TemplateFile {
                file_name: format!("{name}.routes.ts"),
                content: get_content(include_str!("statics/routes.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: format!("{name}.controller.ts"),
                content: get_content(include_str!("statics/controller.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: format!("{name}.service.ts"),
                content: get_content(include_str!("statics/service.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: format!("{name}.logic.ts"),
                content: get_content(include_str!("statics/logic.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: format!("{name}.repository.ts"),
                content: get_content(include_str!("statics/repository.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: format!("{name}.interface.ts"),
                content: get_content(include_str!("statics/interface.txt"), name),
                path: path.clone(),
            },
            TemplateFile {
                file_name: "validation/schema.ts".to_string(),
                content: include_str!("statics/schema.txt").to_string(),
                path: format!("{path}/validation"),
            },
        ],

        Flag::Routes(name) => vec![TemplateFile {
            file_name: format!("{name}.routes.ts"),
            content: get_content(include_str!("statics/routes.txt"), name),
            path,
        }],

        Flag::Controller(name) => vec![TemplateFile {
            file_name: format!("{name}.controller.ts"),
            content: get_content(include_str!("statics/controller.txt"), name),
            path,
        }],

        Flag::Service(name) => vec![TemplateFile {
            file_name: format!("{name}.service.ts"),
            content: get_content(include_str!("statics/service.txt"), name),
            path,
        }],

        Flag::Logic(name) => vec![TemplateFile {
            file_name: format!("{name}.logic.ts"),
            content: get_content(include_str!("statics/logic.txt"), name),
            path,
        }],

        Flag::Repository(name) => vec![TemplateFile {
            file_name: format!("{name}.repository.ts"),
            content: get_content(include_str!("statics/repository.txt"), name),
            path,
        }],

        Flag::Interface(name) => vec![TemplateFile {
            file_name: format!("{name}.interface.ts"),
            content: get_content(include_str!("statics/interface.txt"), name),
            path,
        }],

        Flag::Schema => vec![TemplateFile {
            file_name: "validation/schema.ts".to_string(),
            content: include_str!("statics/schema.txt").to_string(),
            path: format!("{path}/validation"),
        }],

        _ => panic!("Unrecognized flag"),
    }
}
