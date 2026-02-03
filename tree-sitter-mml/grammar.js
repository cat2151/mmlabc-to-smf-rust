module.exports = grammar({
  name: 'mml',

  rules: {
    source_file: $ => choice(
      $.channel_groups,
      repeat($._item)
    ),

    channel_groups: $ => seq(
      $.channel_group,
      repeat1(seq(';', $.channel_group))
    ),

    channel_group: $ => repeat1($._item),

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
      $.velocity_set,
    ),

    chord: $ => seq(
      "'",
      repeat1($.note_with_modifier),
      "'"
    ),

    note_with_modifier: $ => seq(
      $.note,
      optional($.modifier),
      optional($.note_length),
      optional($.dots)
    ),

    note: $ => /[cdefgabCDEFGAB]/,
    modifier: $ => choice('+', '-'),
    note_length: $ => /[0-9]+/,
    dots: $ => /\.+/,
    rest: $ => seq(
      /[rR]/,
      optional($.note_length),
      optional($.dots)
    ),
    octave_up: $ => '<',
    octave_down: $ => '>',
    octave_set: $ => seq('o', /[0-9]+/),
    length_set: $ => seq('l', /[0-9]+/, optional($.dots)),
    program_change: $ => seq('@', /[0-9]+/),
    tempo_set: $ => seq('t', /[0-9]+/),
    velocity_set: $ => seq('v', /[0-9]+/),
  }
});
