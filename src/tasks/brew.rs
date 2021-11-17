use crate::TaskError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, char},
    combinator::{opt, recognize, verify},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum BrewTask {
    Cask { name: String, ver: Option<String> },
    Formulae { name: String, ver: Option<String> },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cache {
    casks: HashMap<String, String>,
    formulae: HashMap<String, String>,
}

fn parse_installed_package(src: &str) -> IResult<&str, (&str, &str)> {
    let (src, name) = recognize(many1(alt((alphanumeric1, tag("-"), tag("_")))))(src)?;
    let (src, _) = char(' ')(src)?;
    let (src, (ver, _)) = tuple((
        recognize(many1(verify(anychar, |c| *c != '\n'))),
        opt(tag("\n")),
    ))(src)?;
    Ok((src, (name, ver)))
}

fn parse_brew_list(src: &str) -> IResult<&str, HashMap<String, String>> {
    let (src, packages) = many0(parse_installed_package)(src)?;
    Ok((
        src,
        packages
            .into_iter()
            .map(|(name, ver)| (name.to_owned(), ver.to_owned()))
            .collect::<HashMap<_, _>>(),
    ))
}

#[cfg(test)]
mod test_brew_list_parser {
    use super::*;

    #[test]
    fn test_parse_installed_packages() {
        assert_eq!(
            parse_installed_package("arm-none-eabi-gcc 10.3-2021.07").unwrap(),
            ("", ("arm-none-eabi-gcc", "10.3-2021.07"))
        );
    }
}

fn calcurate_cache() -> Result<Cache, TaskError> {
    let output_formulae = duct::cmd("brew", &["list", "--formulae", "--versions"])
        .read()
        .map_err(|_| {
            TaskError::WellKnown("cannot fetch installed cargo package information".to_owned())
        })?;
    let output_cask = duct::cmd("brew", &["list", "--casks", "--versions"])
        .read()
        .map_err(|_| {
            TaskError::WellKnown("cannot fetch installed cargo package information".to_owned())
        })?;
    let (_, packages_formulae) = parse_brew_list(&output_formulae).map_err(|_| {
        TaskError::WellKnown(
            "cannot parse installed cargo package information. this is bug".to_owned(),
        )
    })?;
    let (_, packages_cask) = parse_brew_list(&output_cask).map_err(|_| {
        TaskError::WellKnown(
            "cannot parse installed cargo package information. this is bug".to_owned(),
        )
    })?;
    Ok(Cache {
        casks: packages_cask,
        formulae: packages_formulae,
    })
}

impl crate::Task for BrewTask {
    fn name(&self) -> String {
        match self {
            Self::Cask { name, ver: None } => format!("brew cask {}", name),
            Self::Cask {
                name,
                ver: Some(ver),
            } => format!("brew cask {}@{}", name, ver),
            Self::Formulae { name, ver: None } => format!("brew cask {}", name),
            Self::Formulae {
                name,
                ver: Some(ver),
            } => format!("brew cask {}@{}", name, ver),
        }
    }

    fn execute(&self, ctx: &crate::TaskContext) -> crate::TaskResult {
        let packages = if ctx.cache.borrow().is_some() {
            rmp_serde::from_read_ref(ctx.cache.borrow().as_ref().expect("already checked"))
                .map_err(|e| TaskError::Unknown(e.into()))?
        } else {
            let cache = calcurate_cache()?;
            *ctx.cache.borrow_mut() =
                Some(rmp_serde::to_vec(&cache).map_err(|e| TaskError::Unknown(e.into()))?);
            cache
        };
        let cmd = match self {
            BrewTask::Cask { name, ver } => match (ver, packages.casks.get(name)) {
                (Some(ver), Some(ver_)) if ver != ver_ => Some(duct::cmd(
                    "brew",
                    &["install", "--casks", &format!("{}@{}", name, ver)],
                )),
                (Some(ver), None) => Some(duct::cmd(
                    "brew",
                    &["install", "--casks", &format!("{}@{}", name, ver)],
                )),
                (None, None) => Some(duct::cmd("brew", &["install", "--casks", name])),
                (_, _) => None,
            },
            BrewTask::Formulae { name, ver } => match (ver, packages.formulae.get(name)) {
                (Some(ver), Some(ver_)) if ver != ver_ => Some(duct::cmd(
                    "brew",
                    &["install", &format!("{}@{}", name, ver)],
                )),
                (Some(ver), None) => Some(duct::cmd(
                    "brew",
                    &["install", &format!("{}@{}", name, ver)],
                )),
                (None, None) => Some(duct::cmd("brew", &["install", name])),
                (_, _) => None,
            },
        };
        if let Some(cmd) = cmd {
            cmd.read().map_err(|e| {
                TaskError::WellKnown(format!(
                    "cannot install package {} due to {:?}",
                    self.name(),
                    e
                ))
            })?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn parse(obj: &HashMap<String, crate::ast::Value>) -> Result<crate::TaskEntity, crate::Error> {
    let ver = if let Some(v) = obj.get("version") {
        Some(
            v.as_str()
                .ok_or_else(|| {
                    crate::Error::PlaybookLoadFailed("brew.version must be string".to_owned())
                })?
                .to_owned(),
        )
    } else {
        None
    };
    let formula = if let Some(v) = obj.get("formula") {
        Some(
            v.as_str()
                .ok_or_else(|| {
                    crate::Error::PlaybookLoadFailed("brew.formula must be string".to_owned())
                })?
                .to_owned(),
        )
    } else {
        None
    };
    let cask = if let Some(v) = obj.get("cask") {
        Some(
            v.as_str()
                .ok_or_else(|| {
                    crate::Error::PlaybookLoadFailed("brew.cask must be string".to_owned())
                })?
                .to_owned(),
        )
    } else {
        None
    };
    match (cask, formula) {
        (Some(name), None) => Ok(crate::TaskEntity::Brew(BrewTask::Cask { name, ver })),
        (None, Some(name)) => Ok(crate::TaskEntity::Brew(BrewTask::Formulae { name, ver })),
        _ => Err(crate::Error::PlaybookLoadFailed(
            "one of brew.formula or brew.cask is required.".to_owned(),
        )),
    }
}
