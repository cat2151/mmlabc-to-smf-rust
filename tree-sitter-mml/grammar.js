module.exports = grammar({
  name: 'mml',

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.chord,
      $.note,
      $.octave_up,
      $.octave_down,
    ),

    chord: $ => seq(
      "'",
      repeat1($.note),
      "'"
    ),

    note: $ => /[cdefgabCDEFGAB]/,
    octave_up: $ => '<',
    octave_down: $ => '>',
  }
});
