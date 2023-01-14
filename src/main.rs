use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

#[cfg(feature = "debug")]
use std::env;

#[macro_use]
extern crate lazy_static;

use handlebars::{no_escape, Handlebars};

lazy_static! {
    static ref HOST: String = get_host();
}

const SITE_TITLE: &str = "Grauwölfchen's Workroom";
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

// https://rust-lang.github.io/rust-clippy/master/index.html#result_large_err
#[allow(clippy::result_large_err)]
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
    let mut page_data = BTreeMap::new();
    page_data.insert("host".to_string(), HOST.to_string());
    let page = reg.render_template(include_str!("./index.hbs"), &page_data)?;
    let mut data = load("Home", &page);
    data.append(&mut page_data);
    let result = reg.render("layout", &data)?;
    fs::write(dst.join("index.html"), result)?;

    // about
    let mut page_data = BTreeMap::new();
    page_data.insert("host".to_string(), HOST.to_string());
    let page = reg.render_template(include_str!("./about.hbs"), &page_data)?;
    let mut data = load("About", &page);
    data.append(&mut page_data);
    let result = reg.render("layout", &data)?;
    fs::write(dst.join("about.html"), result)?;

    // software
    let mut page_data = BTreeMap::new();
    page_data.insert("host".to_string(), HOST.to_string());
    let page =
        reg.render_template(include_str!("./software.hbs"), &page_data)?;
    let mut data = load("Software", &page);
    data.append(&mut page_data);
    let result = reg.render("layout", &data)?;
    fs::write(dst.join("software.html"), result)?;

    // link
    let mut page_data = BTreeMap::new();
    page_data.insert("host".to_string(), HOST.to_string());
    let page = reg.render_template(include_str!("./link.hbs"), &page_data)?;
    let mut data = load("Link", &page);
    data.append(&mut page_data);
    let result = reg.render("layout", &data)?;
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

    #[cfg(not(feature = "debug"))]
    let href = link.href.to_string();

    #[cfg(feature = "debug")]
    let href = format!(
        "{}{}",
        HOST.to_string(),
        if link.href == "/" {
            "/index.html"
        } else {
            link.href
        }
    );

    format!(
        r#"<li{}>
  <a href="{}">{}</a>
</li>"#,
        class, href, link.title,
    )
}

/// Returns the host part for href value of links.
fn get_host() -> String {
    #[cfg(not(feature = "debug"))]
    let host = "".to_string();

    // for local debugging
    #[cfg(feature = "debug")]
    let host =
        format!("{}/dst", env::current_dir().unwrap().display().to_string());
    host
}

/// Loads the content as a BTreeMap object that has navi, content
/// {page,site}_title as attributes.
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

    #[cfg(not(feature = "debug"))]
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
            title: "Foo",
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

    #[cfg(feature = "debug")]
    #[test]
    fn test_build_nav_debug() {
        let dir = Path::new("/tmp");
        assert!(env::set_current_dir(&dir).is_ok());

        let link = Link {
            title: "Foo",
            href: "/foo.html",
        };
        let nav = build_nav("Foo", &link);
        assert_eq!(
            nav,
            r#"<li class="active">
  <a href="/tmp/dst/foo.html">Foo</a>
</li>"#
        );

        let link = Link {
            title: "Foo",
            href: "/foo.html",
        };
        let nav = build_nav("Bar", &link);
        assert_eq!(
            nav,
            r#"<li>
  <a href="/tmp/dst/foo.html">Foo</a>
</li>"#
        );
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn test_get_host() {
        let host = get_host();
        assert_eq!(host, "");
    }

    #[cfg(feature = "debug")]
    #[test]
    fn test_get_host_debug() {
        let dir = Path::new("/tmp/");
        assert!(env::set_current_dir(&dir).is_ok());

        let host = get_host();
        assert_eq!(host, "/tmp/dst");
    }
}
