module.exports = grammar({
  name: 'mml',

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.chord,
      $.note_with_modifier,
      $.rest,
      $.octave_up,
      $.octave_down,
      $.octave_set,
    ),

    chord: $ => seq(
      "'",
      repeat1($.note_with_modifier),
      "'"
    ),

    note_with_modifier: $ => seq(
      $.note,
      optional($.modifier)
    ),

    note: $ => /[cdefgabCDEFGAB]/,
    modifier: $ => choice('+', '-'),
    rest: $ => /[rR]/,
    octave_up: $ => '<',
    octave_down: $ => '>',
    octave_set: $ => seq('o', /[0-9]+/),
  }
});
