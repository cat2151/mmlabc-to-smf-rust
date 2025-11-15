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
      $.length_set,
      $.program_change,
      $.tempo_set,
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
    length_set: $ => seq('l', /[0-9]+/),
    program_change: $ => seq('@', /[0-9]+/),
    tempo_set: $ => seq('t', /[0-9]+/),
  }
});
