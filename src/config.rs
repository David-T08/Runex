use anyhow::{Context, Result};
use jsonc_parser::parse_to_serde_value;
use serde::Deserialize;
use std::collections::BTreeSet;

use gtk4_layer_shell as gls;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    // pub width: i32,

    // pub query_height: i32,

    #[serde(rename = "overlay-scrim")]
    pub overlay_scrim: OverlayScrim,
}

#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum EdgeDef {
    Left,
    Right,
    Top,
    Bottom,
}

impl From<EdgeDef> for gls::Edge {
    fn from(e: EdgeDef) -> Self {
        match e {
            EdgeDef::Left => Self::Left,
            EdgeDef::Right => Self::Right,
            EdgeDef::Top => Self::Top,
            EdgeDef::Bottom => Self::Bottom,
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", default)]
pub struct Margins {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Margins {
    fn all(v: i32) -> Self {
        Self {
            left: v,
            right: v,
            top: v,
            bottom: v,
        }
    }
    fn set_if_some(&mut self, other: &MarginsOpt) {
        if let Some(v) = other.left {
            self.left = v;
        }
        if let Some(v) = other.right {
            self.right = v;
        }
        if let Some(v) = other.top {
            self.top = v;
        }
        if let Some(v) = other.bottom {
            self.bottom = v;
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case", default)]
pub struct MarginsOpt {
    pub left: Option<i32>,
    pub right: Option<i32>,
    pub top: Option<i32>,
    pub bottom: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarginInput {
    All(i32),
    Per(MarginsOpt),
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct OverlayScrim {
    /// Shorthand: all four anchors
    pub fullscreen: bool,

    /// Explicit anchors (any subset). If `fullscreen` or `fullscreen-margin`
    /// is present, those take precedence.
    pub anchors: Option<Vec<EdgeDef>>,

    /// Shorthand: same margin on all edges
    pub margin: Option<i32>,

    /// Per-edge margins (each field optional)
    pub margins: Option<MarginsOpt>,

    /// Shorthand: fullscreen + margins (either a number or per-edge object)
    pub fullscreen_margin: Option<MarginInput>,
}

// ---------------- Resolution ----------------

#[derive(Debug, Clone)]
pub struct ResolvedScrim {
    pub edges: Vec<EdgeDef>, // deduped, sorted
    pub margins: Margins,    // concrete per-edge margins
}

impl OverlayScrim {
    pub fn resolve(&self) -> ResolvedScrim {
        let mut edges: BTreeSet<EdgeDef> = BTreeSet::new();

        if self.fullscreen || self.fullscreen_margin.is_some() {
            edges.extend([EdgeDef::Left, EdgeDef::Right, EdgeDef::Top, EdgeDef::Bottom]);
        } else if let Some(a) = &self.anchors {
            edges.extend(a.iter().cloned());
        }

        let mut margins = Margins::default();

        // fullscreen-margin overrides both margin & margins
        if let Some(fm) = &self.fullscreen_margin {
            match fm {
                MarginInput::All(v) => margins = Margins::all(*v),
                MarginInput::Per(per) => {
                    margins = Margins::default();
                    margins.set_if_some(per);
                }
            }
        } else {
            if let Some(v) = self.margin {
                margins = Margins::all(v);
            }
            if let Some(per) = &self.margins {
                margins.set_if_some(per);
            }
        }

        ResolvedScrim {
            edges: edges.into_iter().collect(),
            margins,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    // pub width: i32,
    // pub query_height: i32,
    pub scrim: ResolvedScrim,
}

impl Configuration {
    pub fn resolve(&self) -> ResolvedConfig {
        ResolvedConfig {
            // width: self.width,
            // query_height: self.query_height,
            scrim: self.overlay_scrim.resolve(),
        }
    }
}

pub fn config_home_path() -> PathBuf {
    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        if !path.is_empty() {
            return path.into();
        }
    }

    let home = env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".config")
}

pub fn from_env_or_home() -> Result<ResolvedConfig> {
    let path = config_home_path().join("runex").join("config.jsonc");

    return from_file(&path);
}

pub fn from_file(path: &Path) -> Result<ResolvedConfig> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read configuration file at {}", path.display()))?;

    let raw = parse_to_serde_value(&contents, &Default::default())?
        .ok_or_else(|| anyhow::anyhow!("Invalid configuration file"))?;

    let config: Configuration =
        serde_json::from_value(raw).context("Failed to deserialize configuration file")?;

    Ok(config.resolve())
}
