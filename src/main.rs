use std::fs;
use std::io;
use std::path::Path;
use std::collections::BTreeMap;

use handlebars::{Handlebars, no_escape};

const SITE_TITLE: &str = "Grauwoelfchen's Workroom";
const DST_DIR: &str = "./dst";

const LINKS: [(&str, &str); 4] = [
    ("Home", "/"),
    ("About", "/about"),
    ("Software", "/software"),
    ("Link", "/link"),
];

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Parse(handlebars::TemplateError),
    Render(handlebars::RenderError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(error: handlebars::TemplateError) -> Self {
        Self::Parse(error)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(error: handlebars::RenderError) -> Self {
        Self::Render(error)
    }
}

fn main() -> Result<(), Error> {
    let mut reg = Handlebars::new();

    let layout = include_str!("./layout.hbs");
    reg.register_template_string("layout", layout)?;
    reg.register_escape_fn(no_escape);

    let dst = Path::new(&DST_DIR);

    // error
    let dat = include_str!("./error.hbs");
    fs::write(dst.join("error.html"), &dat.as_bytes())?;

    // index
    let data = load("Home", include_str!("./index.hbs"));
    let result = reg.render("layout", &data)?;
    fs::write(dst.join("index.html"), result)?;

    // about
    let data = load("About", include_str!("./about.hbs"));
    let result = reg.render("layout", &data).unwrap();
    fs::write(dst.join("about.html"), result)?;

    // software
    let data = load("Software", include_str!("./software.hbs"));
    let result = reg.render("layout", &data).unwrap();
    fs::write(dst.join("software.html"), result)?;

    // link
    let data = load("Link", include_str!("./link.hbs"));
    let result = reg.render("layout", &data).unwrap();
    fs::write(dst.join("link.html"), result)?;

    reg.unregister_escape_fn();

    Ok(())
}

fn load(heading: &str, content: &str) -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();

    let mut nav = "".to_string();
    for link in LINKS {
        let class = if link.0 == heading {
            "active"
        } else {
            ""
        };
        let n = format!(
            "<li title=\"{}\" href=\"{}\" class=\"{}\">{}</li>",
            link.0,
            link.1,
            class,
            link.0,
        );
        nav = format!("{}{}", &nav, &n);
    }
    data.insert("navi".to_string(), nav);
    data.insert("content".to_string(), content.to_string());

    // meta
    if heading != "Home" {
        data.insert("page_title".to_string(), heading.to_string());
    }
    data.insert("site_title".to_string(), SITE_TITLE.to_string());
    data
}
