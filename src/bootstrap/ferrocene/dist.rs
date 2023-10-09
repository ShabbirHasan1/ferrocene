// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::config::TargetSelection;
use crate::tarball::{GeneratedTarball, Tarball};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Docs {
    target: TargetSelection,
}

impl Step for Docs {
    type Output = Vec<GeneratedTarball>;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias("ferrocene-docs").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Docs { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        // Build all of the documentation.
        builder.default_doc(&[]);
        let doc_out = builder.out.join(&self.target.triple).join("doc");

        let mut subsetter = Subsetter::new(builder, "ferrocene-docs", "share/doc/ferrocene/html");
        subsetter.add_directory(&doc_out, &doc_out);

        subsetter.into_tarballs().map(|tarball| tarball.generate()).collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SourceTarball;

impl Step for SourceTarball {
    type Output = Vec<GeneratedTarball>;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.alias("ferrocene-src").default_condition(builder.config.rust_dist_src)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SourceTarball);
    }

    fn run(self, builder: &Builder<'_>) -> Vec<GeneratedTarball> {
        // Configuration of what should be included in the tarball.
        const DIRS: &[&str] =
            &["src", "compiler", "library", "tests", "ferrocene", "LICENSES", ".reuse"];
        const FILES: &[&str] = &[
            "COPYRIGHT",
            "LICENSE-APACHE",
            "LICENSE-MIT",
            "README.md",
            "RELEASES.md",
            "config.example.toml",
            "Cargo.toml",
            "Cargo.lock",
            "configure",
            "x",
            "x.py",
            "x.ps1",
        ];
        const EXTRA_CARGO_TOMLS: &[&str] = &[
            "compiler/rustc_codegen_cranelift/Cargo.toml",
            "src/bootstrap/Cargo.toml",
            "src/tools/rust-analyzer/Cargo.toml",
        ];

        let mut subsetter = Subsetter::new(builder, "ferrocene-src", "");

        // Copy raw source files
        for item in DIRS {
            subsetter.add_directory(&builder.src, &builder.src.join(item));
        }
        for item in FILES {
            subsetter.add_file(&builder.src, &builder.src.join(item));
        }

        let generic_tarball = subsetter
            .tarballs
            .get(&None)
            .expect("generic tarball was not generated, all the files were part of a subset")
            .clone();
        let dest_dir = &generic_tarball.image_dir();

        // Include metadata about the git commit. This will be picked up by bootstrap when building
        // Ferrocene from the tarball, so that the final build will include the right git commit
        // even though it didn't come from the repository.
        if let Some(info) = builder.rust_info().info() {
            crate::channel::write_commit_info_file(&dest_dir, info);
        }

        // Vendor Rust dependencies
        let mut vendor = Command::new(&builder.initial_cargo);
        vendor.arg("vendor").arg("vendor/rust").current_dir(&dest_dir);
        vendor.env("RUSTC_BOOTSTRAP", "1"); // std's Cargo.toml uses unstable features
        for extra in EXTRA_CARGO_TOMLS {
            vendor.arg("--sync").arg(&builder.src.join(extra));
        }
        if !builder.config.dry_run() {
            let config = crate::util::output(&mut vendor);
            builder.create_dir(&dest_dir.join(".cargo"));
            builder.create(&dest_dir.join(".cargo").join("config.toml"), &config);
        }

        drop(generic_tarball);
        subsetter
            .into_tarballs()
            .map(|mut tarball| {
                tarball.permit_symlinks(true);
                tarball.bare()
            })
            .collect()
    }
}

struct Subsetter<'a> {
    builder: &'a Builder<'a>,
    name_prefix: String,
    output_prefix: PathBuf,

    tarballs: BTreeMap<Option<String>, Rc<Tarball<'a>>>,
    current_subset: Option<String>,
}

impl<'a> Subsetter<'a> {
    fn new(builder: &'a Builder<'a>, name_prefix: &str, output_prefix: &str) -> Self {
        Self {
            builder,
            name_prefix: name_prefix.into(),
            output_prefix: output_prefix.into(),
            tarballs: BTreeMap::new(),
            current_subset: None,
        }
    }

    fn add_directory(&mut self, root: &Path, path: &Path) {
        self.with_tarball(path, |this, tarball| {
            for entry in std::fs::read_dir(path).unwrap() {
                let path = entry.as_ref().unwrap().path();

                if path.is_file() {
                    let relative = path.strip_prefix(root).unwrap();
                    let mode = if this.is_executable(&path) { 0o755 } else { 0o644 };
                    tarball.add_file(
                        &path,
                        this.output_prefix.join(relative).parent().unwrap(),
                        mode,
                    );
                } else if path.is_dir() {
                    this.add_directory(root, &path);
                }
            }
        })
    }

    fn add_file(&mut self, root: &Path, path: &Path) {
        self.with_tarball(path.parent().unwrap(), |this, tarball| {
            let relative = path.strip_prefix(root).unwrap();
            let mode = if this.is_executable(&path) { 0o755 } else { 0o644 };
            tarball.add_file(&path, this.output_prefix.join(relative).parent().unwrap(), mode);
        });
    }

    fn with_tarball<F: FnOnce(&mut Self, &Tarball<'a>)>(&mut self, path: &Path, f: F) {
        let old_subset = self.current_subset.clone();

        let subset_file = path.join("ferrocene-subset");
        match std::fs::read_to_string(&subset_file) {
            Ok(data) => self.current_subset = Some(self.parse_subset_file(&subset_file, &data)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => panic!("failed to read ferrocene-subset in {}: {err}", path.display()),
        }
        let tarball = match self.tarballs.get(&self.current_subset) {
            Some(tarball) => tarball.clone(),
            None => {
                let name = match &self.current_subset {
                    Some(name) => format!("{}-{name}", self.name_prefix),
                    None => self.name_prefix.clone(),
                };
                let tarball = Rc::new(Tarball::new_targetless(self.builder, &name));
                self.tarballs.insert(self.current_subset.clone(), tarball.clone());
                tarball
            }
        };

        f(self, &tarball);

        self.current_subset = old_subset;
    }

    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::metadata(path).unwrap().permissions().mode() & 0o111 > 0
    }
    #[cfg(not(unix))]
    fn is_executable(&self, _: &Path) -> bool {
        panic!("generating accurate tarballs on non-unix-like platforms is not yet supported");
    }

    fn parse_subset_file(&self, path: &Path, contents: &str) -> String {
        let mut lines = contents
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#'))
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());

        let Some(subset) = lines.next() else {
            panic!("no content in subset file {}", path.display());
        };
        if !subset.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            panic!("subset name {subset:?} contains invalid chars (in {})", path.display());
        }
        if lines.next().is_some() {
            panic!("multiple subset names in {}", path.display());
        }

        subset.into()
    }

    fn into_tarballs(self) -> impl Iterator<Item = Tarball<'a>> {
        self.tarballs.into_values().map(|tarball| Rc::try_unwrap(tarball).map_err(|_| ()).unwrap())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SelfTest {
    pub(crate) target: TargetSelection,
}

impl Step for SelfTest {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let self_test = builder.ensure(crate::ferrocene::tool::SelfTest { target: self.target });

        let tarball = Tarball::new(builder, "ferrocene-self-test", &self.target.triple);
        tarball.add_file(self_test, "bin", 0o755);
        tarball.generate()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TestOutcomes;

impl Step for TestOutcomes {
    type Output = Option<GeneratedTarball>;
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-test-outcomes")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TestOutcomes);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let Some(test_outcomes) = &builder.config.ferrocene_test_outcomes_dir else { return None };

        let tarball = Tarball::new_targetless(builder, "ferrocene-test-outcomes");
        tarball.add_dir(test_outcomes, "share/ferrocene/test-outcomes");
        Some(tarball.generate())
    }
}
