module.exports = grammar({
  name: 'mml',

  rules: {
    source_file: $ => repeat($._item),

    _item: $ => choice(
      $.note,
      $.octave_up,
      $.octave_down,
    ),

    note: $ => /[cdefgabCDEFGAB]/,
    octave_up: $ => '<',
    octave_down: $ => '>',
  }
});
