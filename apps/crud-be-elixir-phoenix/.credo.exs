%{
  configs: [
    %{
      name: "default",
      files: %{
        included: ["lib/", "test/"],
        excluded: []
      },
      strict: true,
      color: true,
      checks: [
        {Credo.Check.Design.AliasUsage, false},
        {Credo.Check.Readability.Specs, false},
        # Cabbage BDD step files use ConnCase but are named *_steps.exs, not *_test.exs
        {Credo.Check.Warning.WrongTestFilename, false},

        # @covers <spec-path>:<scenario-title> marker comments in test/{unit,integration}/steps/*.exs
        # must stay on one line for rhino-cli's spec-coverage regex matching; the default 120
        # doesn't fit long spec paths.
        {Credo.Check.Readability.MaxLineLength, [max_length: 200]}
      ]
    }
  ]
}
