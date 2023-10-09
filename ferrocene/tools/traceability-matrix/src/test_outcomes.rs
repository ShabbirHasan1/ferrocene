// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use anyhow::{Context, Error};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

const COMPILETEST_TYPE: &str = "bootstrap::test::Compiletest";

const EXPECTED_FORMAT_VERSION: usize = 1;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct TestOutcomes {
    // key is name of test, represented by a path
    // value is targets on which the tests were executed
    pub(crate) executed_tests: BTreeMap<String, BTreeSet<String>>,
    // key is name of test, also represented by a path
    // value can be any number of ignored tests
    pub(crate) ignored_tests: BTreeMap<String, BTreeSet<String>>,
}

impl TestOutcomes {
    pub(crate) fn load(directory: &Path) -> Result<Self, Error> {
        let mut test_outcomes = TestOutcomes::default();

        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let metrics: Metrics = serde_json::from_slice(&std::fs::read(&path)?)
                .with_context(|| format!("failed to read metrics from {}", path.display()))?;
            if metrics.format_version != EXPECTED_FORMAT_VERSION {
                anyhow::bail!(
                    "build metrics format version {} is not supported",
                    metrics.format_version
                );
            }

            let mut search_queue = VecDeque::new();
            for invocation in &metrics.invocations {
                search_queue.extend(invocation.children.iter().map(|c| (c, false)));
            }
            while let Some((node, mut inside_compiletest)) = search_queue.pop_front() {
                match node {
                    MetricsNode::RustbuildStep { type_, children } => {
                        if type_ == COMPILETEST_TYPE {
                            inside_compiletest = true;
                        }
                        for child in children {
                            search_queue.push_back((child, inside_compiletest));
                        }
                    }
                    MetricsNode::TestSuite(TestSuite { tests, metadata }) => {
                        if !inside_compiletest {
                            continue;
                        }

                        for Test { name, outcome } in tests {
                            // Compiletest test names are in the "[suite] path/to/test.rs#revision"
                            // format, with the revision being optional.
                            let Some(name) = name.split_once("] ").map(|(_, n)| n) else {
                                continue;
                            };
                            let name = name.rsplit_once('#').map(|(n, _)| n).unwrap_or(name).into();

                            let (TestSuiteMetadata::CargoPackage { target }
                            | TestSuiteMetadata::Compiletest { target }) = metadata;

                            if let MetricsTestOutcome::Ignored = outcome {
                                test_outcomes
                                    .ignored_tests
                                    .entry(name)
                                    .or_insert_with(BTreeSet::new)
                                    .insert(target.to_owned());
                            } else {
                                test_outcomes
                                    .executed_tests
                                    .entry(name)
                                    .or_insert_with(BTreeSet::new)
                                    .insert(target.to_owned());
                            }
                        }
                    }
                }
            }
        }

        Ok(test_outcomes)
    }
}

#[derive(serde::Deserialize)]
struct Metrics {
    #[serde(default)] // The field was not present in version 0
    format_version: usize,
    invocations: Vec<MetricsInvocation>,
}

#[derive(serde::Deserialize)]
struct MetricsInvocation {
    children: Vec<MetricsNode>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum MetricsNode {
    RustbuildStep {
        #[serde(rename = "type")]
        type_: String,
        children: Vec<MetricsNode>,
    },
    TestSuite(TestSuite),
}

#[derive(serde::Deserialize)]
struct TestSuite {
    metadata: TestSuiteMetadata,
    tests: Vec<Test>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum TestSuiteMetadata {
    CargoPackage { target: String },
    Compiletest { target: String },
}

#[derive(serde::Deserialize)]
struct Test {
    name: String,
    outcome: MetricsTestOutcome,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// Failed is missing so as to trigger a deserialize failure,
// because we should not reach this far if any test fails.
enum MetricsTestOutcome {
    Passed,
    Ignored,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_load_outcomes() {
        let dir = TempDir::new().unwrap();

        let content = json!({
            "format_version": 1,
            "invocations": [
                {
                    "children": [
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::compile::Assemble",
                            "children": [],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Ui",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "cargo_package",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[ui] tests/ui/foo.rs",
                                                    "outcome": "passed",
                                                },
                                                {
                                                    "name": "[ui] tests/ui/bar.rs",
                                                    "outcome": "passed",
                                                },
                                                {
                                                    "name": "[ui] tests/ui/baz.rs",
                                                    "outcome": "ignored",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::RunMake",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "compiletest",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[run-make] tests/run-make/foo.rs#revision",
                                                    "outcome": "passed",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Crate",
                            "children": [
                                {
                                    "kind": "test_suite",
                                    "metadata": {
                                        "target": "aarch64-unknown-linux-gnu",
                                        "kind": "compiletest",
                                    },
                                    "tests": [
                                        {
                                            "name": "rustc_foo::bar::tests::test_baz",
                                            "outcome": "passed",
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
        .to_string();
        let content: serde_json::Value = serde_json::from_str(&content).unwrap();
        // for ease of debugging
        let content = serde_json::to_string_pretty(&content).unwrap();
        std::fs::write(dir.path().join("runner1.json"), content).unwrap();

        let content = json!({
            "format_version": 1,
            "invocations": [
                {
                    "children": [
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Codegen",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "compiletest",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[codegen] tests/codegen/foo.rs",
                                                    "outcome": "passed",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                    ],
                },
                {
                    "children": [
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Codegen",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "compiletest",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[codegen] tests/codegen/foo.rs",
                                                    "outcome": "passed",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                    ],
                },
            ],
        })
        .to_string();
        let content: serde_json::Value = serde_json::from_str(&content).unwrap();
        // for ease of debugging
        let content = serde_json::to_string_pretty(&content).unwrap();
        std::fs::write(dir.path().join("runner2.json"), content).unwrap();

        let outcomes = TestOutcomes::load(dir.path()).unwrap();

        assert_eq!(
            TestOutcomes {
                executed_tests: BTreeMap::from([
                    (
                        "tests/ui/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/ui/bar.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/run-make/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/codegen/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                ]),
                ignored_tests: BTreeMap::from([(
                    "tests/ui/baz.rs".into(),
                    BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                )]),
            },
            outcomes,
        )
    }
}
