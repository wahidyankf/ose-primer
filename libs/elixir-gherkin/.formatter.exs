[
  # 130: fits test/gherkin/feature_parsing_behavior_test.exs's defgiven/defwhen/defthen
  # calls (literal Gherkin step-text regexes) on one line — mix format wrapping them
  # into parenthesized multi-line form breaks rhino-cli's ex_step_re() extractor,
  # which requires "defgiven"/"defwhen"/"defthen" followed directly by whitespace
  # then "~r/...", not "defgiven(\n  ~r/...".
  line_length: 130,
  inputs: ["mix.exs", "{config,lib,test}/**/*.{ex,exs}"]
]
