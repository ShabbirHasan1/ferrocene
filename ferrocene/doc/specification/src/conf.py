# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Ferrous Systems and AdaCore

# -- Path setup --------------------------------------------------------------

import os
import sys

sys.path.insert(0, os.path.abspath("../exts"))
sys.path.insert(1, os.path.abspath("../shared/exts"))


# -- Project information -----------------------------------------------------

project = "Ferrocene Language Specification"
copyright = "Ferrous Systems and AdaCore"
author = "Ferrous Systems and AdaCore"


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "ferrocene_spec",
    "ferrocene_spec_lints",
    "ferrocene_toctrees",
]

# Add any paths that contain templates here, relative to this directory.
templates_path = []

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = []

rst_prolog = """
.. caution::

   You're reading a draft of the Ferrocene Language Specification. Some parts
   of this document might be missing, incomplete or incorrect. Our aim is to
   have the specification ready by the end of 2022.
"""


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#

html_theme = "ferrocene"
html_theme_path = ["../shared/themes"]

html_theme_options = {
    "license": "MIT or Apache 2.0",
}

html_title = "Ferrocene Language Specification"
html_short_title = "Language Specification"

# -- Options for linting -----------------------------------------------------

lint_alphabetical_section_titles = ["glossary"]

lint_no_paragraph_ids = ["index"]
