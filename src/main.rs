use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use handlebars::{no_escape, Handlebars};

const SITE_TITLE: &str = "Grauwoelfchen's Workroom";
const DST_DIR: &str = "./dst";

const LINKS: [Link; 4] = [
    Link {
        title: "Home",
        href: "/",
    },
    Link {
        title: "About",
        href: "/about.html",
    },
    Link {
        title: "Software",
        href: "/software.html",
    },
    Link {
        title: "Link",
        href: "/link.html",
    },
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

struct Link {
    pub title: &'static str,
    pub href: &'static str,
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

/// Returns built an entry for navigations list.
fn build_nav(heading: &str, link: &Link) -> String {
    let class = if link.title == heading {
        " class=\"active\""
    } else {
        ""
    };
    format!(
        r#"<li{}>
  <a href="{}">{}</a>
</li>"#,
        class, link.href, link.title,
    )
}

fn load(heading: &str, content: &str) -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();

    let mut nav = "".to_string();
    for link in LINKS {
        nav.push_str(&build_nav(heading, &link));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_nav() {
        let link = Link {
            title: "Foo",
            href: "/foo.html",
        };
        let nav = build_nav("Foo", &link);
        assert_eq!(
            nav,
            r#"<li class="active">
  <a href="/foo.html">Foo</a>
</li>"#
        );

        let link = Link {
            title: "Bar",
            href: "/foo.html",
        };
        let nav = build_nav("Bar", &link);
        assert_eq!(
            nav,
            r#"<li>
  <a href="/foo.html">Foo</a>
</li>"#
        );
    }
}
